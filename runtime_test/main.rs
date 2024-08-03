use anyhow::{anyhow, Error, Result};
use async_worker::{Queue, QueueBuilder, Worker};
use axum::{extract::State, routing::post, Json, Router};
use basic_ui::get_basic_ui;
use bindings::ThemeManager;
use log::info;
use objection::{handle_request, RootUi, UiResponse};
use serde_json::Value;
use theme::get_theme;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod basic_ui;
mod bindings;
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
		let body = get_basic_ui(request.get_client().ui());

		request.set_root_ui(ThemeManager::new(get_theme(), body));

		Ok(request.into_response())
	}

	async fn destroy(self) {}
}

#[tokio::main]
async fn main() {
	env_logger::init();

	let queue = leak(QueueBuilder::default().build::<Session>(()));
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

async fn cycle_event_loop(queue: &Queue<Session>, session_id: String, ui: RootUi) -> Result<UiResponse> {
	queue.enqueue(&session_id, ui).await.map_err(map_async_worker_error)?;
	queue.poll(&session_id).await.map_err(map_async_worker_error)?
}

fn map_async_worker_error(error: async_worker::Error) -> Error {
	match error {
		async_worker::Error::WorkerAtCapacity => anyhow!("Slow down a little!! You've been rate-limited."),
		async_worker::Error::NoWorker => anyhow!("No session is associated with the mentioned session id."),
		async_worker::Error::Ceeded => anyhow!("Ceeding response to a newer request on the same session."),
		async_worker::Error::WorkerTerminated => anyhow!("Your session has been closed."),
		async_worker::Error::Timeout => anyhow!("Poll has timed out. Please try again."),
	}
}

fn leak<T>(value: T) -> &'static T {
	&*Box::leak(Box::new(value))
}
