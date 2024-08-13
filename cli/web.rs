use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use axum::{
	extract::{ws::Message, WebSocketUpgrade},
	http::HeaderMap,
	response::Html,
	routing::get,
	serve, Router,
};
use axum_extra::TypedHeader;
use log::{debug, info, warn};
use rand::random;
use std::collections::HashMap;
use tokio::{net::TcpListener, select, sync::mpsc};
use url::Url;

use crate::{
	asset_loader::AssetKind,
	build::{build, Build, BuildOptions},
	diagnostic::DiagnosticList,
	tcp_watcher::{TcpState, TcpWatcher},
	writer::{FileWriter, Writer},
};

#[derive(Debug)]
enum DevConnectionMessage {
	Connected {
		id: u64,
		sender: mpsc::Sender<DevRefreshMessage>,
		user_agent: String,
	},
	Disconnected(u64),
}

#[derive(Debug, Clone, Copy)]
enum DevRefreshMessage {
	FullReload,
	HotReload,
}

#[derive(Debug, Clone, Copy)]
pub struct RunWebStaticParams<'a> {
	pub build_options: BuildOptions<'a>,
	pub web_port: u16,
	pub reload: bool,
	pub bindings_writer: &'a FileWriter,
}

pub async fn run_web_static(params: RunWebStaticParams<'_>) -> Result<()> {
	let mut diagnostic_list = DiagnosticList::new();
	let Build {
		client_bundle,
		bindings,
		assets_loader,
	} = build(&mut diagnostic_list, params.build_options).await?;

	let index = get_index_html(params.build_options.engine_url, true);
	let (dev_connection_sender, mut dev_connection_receiver) = mpsc::channel(10);

	params.bindings_writer.write(bindings).await?;

	let app = Router::new()
		.route("/", get(move || async { Html(index) }))
		.route(
			"/bundle.js",
			get(|| async {
				let mut headers = HeaderMap::new();
				headers.insert("content-type", "appliaction/json".parse().unwrap());

				(headers, client_bundle)
			}),
		)
		.route(
			"/dev.ws",
			get(move |ws: WebSocketUpgrade, user_agent: Option<TypedHeader<headers::UserAgent>>| async move {
				ws.on_upgrade(move |mut socket| async move {
					let id = random();
					let user_agent = user_agent
						.map(|agent| agent.as_str().to_string())
						.unwrap_or("[unspecified user agent]".to_string());

					debug!("recieved dev connection from {user_agent}, labeling it with id {id}");

					let (sender, mut receiver) = mpsc::channel(1);

					if let Ok(_) = dev_connection_sender.send(DevConnectionMessage::Connected { id, sender, user_agent }).await {
						loop {
							let message = select! {
								message = receiver.recv() => match message {
									Some(message) => message,
									None => break,
								},
								message = socket.recv() => match message {
									Some(Ok(Message::Close(_))) => break,
									Some(_) => {
										warn!("Recieved invalid message from client over dev socket: {message:?}");
										continue;
									}
									None => break
								}
							};

							let notification = match message {
								DevRefreshMessage::FullReload => "reload",
								DevRefreshMessage::HotReload => "remount",
							};

							if let Err(_) = socket.send(Message::Text(notification.to_string())).await {
								debug!("socket for dev connection {id} appears to be closed");

								break;
							}
						}
					}

					debug!("about to close dev connection {id}");

					if let Err(_) = dev_connection_sender.send(DevConnectionMessage::Disconnected(id)).await {
						debug!("failed to send disconnect message; server has probably been terminated");
					}
				})
			}),
		);

	let listener = TcpListener::bind(("localhost", params.web_port))
		.await
		.with_context(|| format!("failed to bind to localhost:{}", params.web_port))?;

	info!("Serving the static website at http://localhost:{}", params.web_port);

	if params.reload {
		if let Some(port) = params.build_options.engine_url.port() {
			let url_text = params.build_options.engine_url.to_string();

			tokio::spawn(async move {
				let mut watcher = TcpWatcher::new(port);
				let mut clients = HashMap::<u64, mpsc::Sender<DevRefreshMessage>>::new();

				loop {
					select! {
						change = watcher.next_change() => {
							let change = match change {
								Some(change) => change,
								None => break,
							};

							match change {
								TcpState::Connected => info!("Engine is online at {url_text}"),
								TcpState::Disconnected => (),
								TcpState::Reconnected => {
									info!("Engine has restarted. Triggering a hot-reload in {} client{}", clients.len(), if clients.len() == 1 { "" } else { "s" });

									for (id, sender) in &clients {
										if let Err(_) = sender.send(DevRefreshMessage::HotReload).await {
											debug!("couldn't send refresh message to client {id}; the socket was probably closed at nearly the same time as the reload was triggered");
										}
									}
								},
							}
						}
						connection_message = dev_connection_receiver.recv() => {
							let message = match connection_message {
								Some(message) => message,
								None => {
									debug!("dev_connection sender gave None; suspecting that the dev server was terminated");
									break;
								}
							};

							match message {
								DevConnectionMessage::Connected{ id, sender, user_agent } => {
									clients.insert(id, sender);
									info!("Dev connection ({id}) received from {user_agent}; total connections: {}", clients.len());
								}
								DevConnectionMessage::Disconnected(id) => {
									info!("Dev connection ({id}) has been closed; total connections: {}", clients.len());
									clients.remove(&id);
								}
							};
						}
					};
				}
			});
		} else {
			warn!("Engine url is not local, or does not have an explicit port specification. Disabling hot-reloading.");
		}
	}

	serve(listener, app).await.context("failed to serve the generated web static platform code")?;

	Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct BuildWebStaticParams<'a> {
	pub build_options: BuildOptions<'a>,
	pub bindings_writer: &'a FileWriter,
	pub output_writer: &'a Writer,
}

pub async fn build_web_static(params: BuildWebStaticParams<'_>) -> Result<()> {
	let mut diagnostic_list = DiagnosticList::new();
	let Build {
		client_bundle,
		bindings,
		assets_loader,
	} = build(&mut diagnostic_list, params.build_options).await?;

	params.bindings_writer.write(bindings).await?;
	params
		.output_writer
		.write_file("index.html", get_index_html(params.build_options.engine_url, false))
		.await?;
	params.output_writer.write_file("bundle.js", client_bundle).await?;

	assets_loader.write(params.output_writer, AssetKind::All, &mut diagnostic_list).await?;
	diagnostic_list.flush("write assets")?;

	Ok(())
}

const STATIC_HTML: &str = include_str!("web_index.html");
const DEV_JS: &str = include_str!("dev.js");

fn get_index_html(engine_url: &Url, is_dev: bool) -> String {
	AhoCorasick::new(&["ENGINE_URL", "\"DEV_SCRIPT\""])
		.unwrap()
		.replace_all(STATIC_HTML, &[engine_url.as_str(), if is_dev { DEV_JS } else { "" }])
}
