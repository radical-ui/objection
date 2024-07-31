use anyhow::{bail, Error, Result};
use axum::{extract::State, routing::post, Json, Router};
use log::info;
use objection::{handle_request, RootUi};
use serde_json::Value;
use session_manager::{EnqueueResult, PollResult, Queue, QueueBuilder, Worker};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod bindings;
mod session_manager;

struct Session {}

impl Worker for Session {
	type Context = ();
	type Request = RootUi;
	type Response = Result<RootUi, Error>;
	type Id = String;

	async fn create(id: &Self::Id, context: Self::Context) -> Self {
		Session {}
	}

	async fn handle(&mut self, request: Self::Request) -> Self::Response {
		Ok(request)
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

async fn cycle_event_loop(queue: &Queue<Session>, session_id: String, ui: RootUi) -> Result<RootUi> {
	match queue.enqueue(&session_id, ui) {
		EnqueueResult::WorkerAtCapacity => bail!("Slow down a little!! You've been rate-limited"),
		EnqueueResult::NoWorker => bail!("No session exists for id"),
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
