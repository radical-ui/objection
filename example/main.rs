use anyhow::{bail, Error, Result};
use axum::{extract::State, routing::post, Json, Router};
use bindings::{Component, Label, ThemeManager};
use log::{debug, info};
use objection::{handle_request, RootUi, UiResponse};
use serde_json::Value;
use session_manager::{EnqueueResult, PollResult, Queue, QueueBuilder, Worker};
use theme::get_theme;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod bindings;
mod session_manager;
mod theme;

struct Session {}

impl Worker for Session {
	type Context = ();
	type Request = RootUi;
	type Response = Result<UiResponse, Error>;
	type Id = String;

	async fn create(id: &Self::Id, context: Self::Context) -> Self {
		Session {}
	}

	async fn handle(&mut self, mut request: Self::Request) -> Self::Response {
		request.set_root_ui(ThemeManager::new(get_theme(), Label::new().text("Hello, world")));

		Ok(request.into_response())
	}

	async fn destroy(self) {}
}

#[tokio::main]
async fn main() {
	env_logger::init();

	let queue = leak(QueueBuilder::default().build::<Session>());
	let port = 8000;

	let app = Router::new()
		.route(
			"/ui",
			post(move |State(queue): State<_>, Json(body): Json<Value>| async move {
				Json(handle_request(body, |session_id, ui| cycle_event_loop(queue, session_id, ui)).await)
			}),
		)
		.layer(CorsLayer::very_permissive())
		.with_state(queue);

	let listener = TcpListener::bind(("localhost", port)).await.unwrap();
	info!("listening at http://localhost:{port}");

	axum::serve(listener, app).await.unwrap();
}

async fn cycle_event_loop(queue: &Queue<Session>, session_id: String, mut ui: RootUi) -> Result<UiResponse> {
	if let Some(mount_data) = ui.take_mount_event()? {
		debug!("mounting a new session {session_id}; {mount_data:?}");
		queue.spawn(session_id.clone(), ()).await;
	}

	match queue.enqueue(&session_id, ui) {
		EnqueueResult::WorkerAtCapacity => bail!("Slow down a little!! You've been rate-limited"),
		EnqueueResult::NoWorker => bail!("No session is associated with id {session_id}"),
		EnqueueResult::Sent => (),
	};

	let res = match queue.poll(&session_id).await {
		PollResult::Response(ui) => ui?,
		PollResult::NoWorker => bail!("Session was just terminated"),
		PollResult::WorkerAtCapacity => bail!("Slow down a little!! You've been rate-limited"),
		PollResult::Ceeded => bail!("Ceeding response to a newer request on the same session"),
	};

	Ok(res)
}

fn leak<T>(value: T) -> &'static T {
	&*Box::leak(Box::new(value))
}
