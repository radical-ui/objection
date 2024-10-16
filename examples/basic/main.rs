use anyhow::Result;
use axum::Router;
use log::info;
use objection::{ObjectionService, Session};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	env_logger::init();

	let app = Router::new().route_service("/ui.ws", ObjectionService::<Instance>::new(()));

	let listener = TcpListener::bind(("localhost", 8000)).await.unwrap();
	println!("listening at ws://localhost:8000/ui.ws");

	axum::serve(listener, app).await.unwrap();
}

struct Instance;

impl Session for Instance {
	type Context = ();
	type PeerEvent = ();

	async fn create(_: Option<String>, path: String, _: &Self::Context, controller: objection::Controller<'_>) -> Result<Self> {
		info!("Creating a new session");

		controller.set_object("root_1", json!({ "hello": "this is root 1" }));
		controller.set_object("root_2", json!({ "hello": "this is root 2" }));

		Ok(Instance)
	}

	async fn watch_object(&mut self, id: &str, _: &Self::Context, controller: objection::Controller<'_>) -> Result<()> {
		info!("Requesting to watch object {id}");

		controller.set_object(id, json!({ "hello": "from the backend" }));

		Ok(())
	}
}
