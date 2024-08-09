use axum::{extract::State, routing::post, Json, Router};
use bindings::Label;
use objection::handle_request;
use serde_json::Value;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod bindings;

#[tokio::main]
async fn main() {
	let port = 8000;

	let app = Router::new()
		.route(
			"/ui",
			post(move |State(queue): State<_>, Json(body): Json<Value>| async move {
				Json(
					handle_request(body, |_, ui| async {
						ui.set_root_ui(Label::new("Hello, world!"));
						Ok(ui.into_response())
					})
					.await,
				)
			}),
		)
		.layer(CorsLayer::very_permissive());

	let listener = TcpListener::bind(("localhost", 3000)).await.unwrap();
	println!("listening at http://localhost:3000");

	axum::serve(listener, app).await.unwrap();
}
