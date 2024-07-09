use axum::{
	extract::State,
	http::{HeaderMap, StatusCode},
	response::{IntoResponse, Response},
	routing::{get, put},
	Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use dashmap::DashMap;
use log::{debug, error, info, warn};
use std::{env, future::Future, marker::PhantomData, sync::Arc, time::Duration};
use time::OffsetDateTime;
use tokio::{
	net::TcpListener,
	sync::{mpsc, oneshot},
	time::{sleep, timeout},
};
use uuid::Uuid;

use crate::{Action, Ui, Window};

pub trait HasContext {
	type State: Send + Sync;

	fn create(state: Arc<Self::State>, ui: Ui) -> Self;

	fn ui(&self) -> &Ui;
	fn ui_mut(&mut self) -> &mut Ui;
}

#[allow(unused_variables)]
pub trait HasSession: Sized + Send + Sync + 'static {
	type State: Sync + Send + Clone + 'static;

	fn create(state: Self::State, access_token: Option<&Uuid>, ui: &Ui) -> impl Future<Output = Self> + Send;
	fn get_window(&mut self, state: Self::State, ui: &Ui) -> Window;
	fn update(&mut self, state: Self::State, ui: &mut Ui) -> impl Future + Send;

	fn restore(state: Self::State, id: &Uuid) -> impl Future<Output = Option<Self>> {
		async { None }
	}
	fn save(self, state: Self::State, id: &Uuid) -> impl Future + Send {
		async {}
	}
}

#[derive(Debug)]
struct NewActionMessage {
	action_tree: Vec<Action>,
	responder: oneshot::Sender<Response>,
}

#[derive(Debug)]
struct NewSessionMessage {
	id: Uuid,
	access_token: Option<Uuid>,
	html_responder: oneshot::Sender<String>,
	action_handler: mpsc::Receiver<NewActionMessage>,
}

pub struct Application<Session: HasSession> {
	session_senders: DashMap<Uuid, mpsc::Sender<NewActionMessage>>,
	creation_sender: mpsc::Sender<NewSessionMessage>,
	internal_error_message: String,
	rate_limit_message: String,
	session_queued_operation_limit: usize,
	_marker: PhantomData<Session>,
}

impl<Session: HasSession> Application<Session> {
	pub fn new(state: Session::State) -> Application<Session> {
		let (creation_sender, creation_receiver) = mpsc::channel(10000);

		tokio::spawn(async move {
			drive_application::<Session>(creation_receiver, state).await;
			info!("finished driving the entire set of session lifecycles");
		});

		Application {
			session_senders: DashMap::new(),
			creation_sender,
			internal_error_message: "An internal error has occurred".into(),
			rate_limit_message: "You've been rate-limited".into(),
			session_queued_operation_limit: 5,
			_marker: PhantomData,
		}
	}

	pub async fn listen(self, port: u16) {
		let arc_self = Arc::new(self);
		let cleanup_self = arc_self.clone();

		tokio::spawn(async move {
			loop {
				sleep(Duration::from_secs(60 * 60)).await;
				cleanup_self.drop_old_sessions();
			}
		});

		let app = Router::new()
			.route("/", get(Self::create_session))
			.route("/", put(Self::update_session))
			.with_state(arc_self);

		let listener = TcpListener::bind(("localhost", port)).await.unwrap();
		info!("listening at http://localhost:{port}");

		axum::serve(listener, app).await.unwrap();
	}

	pub async fn create_session(State(application): State<Arc<Application<Session>>>, jar: CookieJar) -> Response {
		let mut headers = HeaderMap::new();
		headers.insert("content-type", "text/html".parse().unwrap());

		let access_token = jar.get("access_token").map(|cookie| Uuid::parse_str(cookie.value()).ok()).flatten();

		let id = Uuid::new_v4();

		// A single session may have up to 5 concurrent tasks running
		let (action_sender, action_handler) = mpsc::channel(application.session_queued_operation_limit);
		application.session_senders.insert(id.clone(), action_sender);

		let (html_responder, html_receiver) = oneshot::channel();
		let send_res = application
			.creation_sender
			.send(NewSessionMessage {
				id,
				access_token,
				action_handler,
				html_responder,
			})
			.await;

		if let Err(_) = send_res {
			error!("application driver stopped or was never started when trying to create a new session");

			return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
		}

		let html = match html_receiver.await {
			Ok(html) => html,
			Err(_) => {
				error!("application driver suddenly stopped while waiting for html");

				return (StatusCode::INTERNAL_SERVER_ERROR,).into_response();
			}
		};

		(headers, html).into_response()
	}

	pub async fn update_session(State(application): State<Arc<Application<Session>>>, headers: HeaderMap, Json(action_tree): Json<Vec<Action>>) -> Response {
		let raw_session_id = match headers.get("x-session-id") {
			Some(session_id) => session_id.to_str().unwrap_or(""),
			None => return (StatusCode::BAD_REQUEST, "Expected an x-session-id header").into_response(),
		};

		let session_id = match Uuid::parse_str(raw_session_id) {
			Ok(uuid) => uuid,
			Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UUID in the x-session-id header").into_response(),
		};

		let session_sender = match application.session_senders.get(&session_id) {
			Some(sender) => sender,
			None => return (StatusCode::NOT_FOUND, "Your session has timed out. Please refresh the page.").into_response(),
		};

		let (responder, receiver) = oneshot::channel();

		if let Err(_) = session_sender.try_send(NewActionMessage { action_tree, responder }) {
			return (StatusCode::TOO_MANY_REQUESTS, application.rate_limit_message.to_string()).into_response();
		}

		match receiver.await {
			Ok(response) => response,
			Err(_) => {
				error!("application driver stopped suddenly for session {}. This is an issue", session_id);

				(StatusCode::INTERNAL_SERVER_ERROR, application.internal_error_message.to_string()).into_response()
			}
		}
	}

	pub fn drop_old_sessions(&self) {
		self.session_senders.retain(|_, sender| !sender.is_closed());
	}
}

async fn drive_application<Session: HasSession>(mut receiver: mpsc::Receiver<NewSessionMessage>, state: Session::State)
where
	Session: Send + Sync + 'static,
{
	let session_timeout = env::var("SESSION_TIMEOUT")
		.ok()
		.map(|var| var.parse::<u64>().ok())
		.flatten()
		.unwrap_or_else(|| {
			warn!("Couldn't find a numeric SESSION_TIMEOUT env var, so falling back to the default");

			60 * 60
		});

	info!("application driver is ready and listening for sessions");

	loop {
		let message = match receiver.recv().await {
			Some(message) => message,
			None => break,
		};

		let state = state.clone();

		tokio::spawn(async move { drive_session::<Session>(message, state, session_timeout).await });
	}
}

async fn drive_session<Session: HasSession>(mut message: NewSessionMessage, state: Session::State, session_timeout: u64) {
	let id = message.id;

	let mut session = {
		debug!("starting session {id}");

		let mut session = Session::create(state.clone(), message.access_token.as_ref(), &Ui::new(Vec::new())).await;

		if let Err(_) = message
			.html_responder
			.send(session.get_window(state.clone(), &Ui::new(Vec::new())).as_html(&id))
		{
			error!("request was unexpectedly closed because we couldn't send the initial html back to the request handler for session {id}");
			return;
		}

		session
	};

	loop {
		let NewActionMessage { action_tree, responder } = match timeout(Duration::from_secs(session_timeout), message.action_handler.recv())
			.await
			.ok()
			.flatten()
		{
			Some(action) => action,
			None => break,
		};

		let state = state.clone();

		let mut ui = Ui::new(action_tree);
		session.update(state.clone(), &mut ui).await;

		let response = match ui.get_access_token_update() {
			Some(token) => {
				let mut cookie = Cookie::new("access_token", token.to_string());
				cookie.set_expires(OffsetDateTime::now_utc() + time::Duration::weeks(2));

				let jar = CookieJar::new().add(cookie);

				(jar, ui.as_json()).into_response()
			}
			None => ui.as_json().into_response(),
		};

		if let Err(_) = responder.send(response) {
			error!("Cannot update because response handler dropped their side of the channel");
		}
	}

	debug!("saving session {}", &message.id);
	session.save(state, &message.id).await;
}
