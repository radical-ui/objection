use anyhow::Result;
use axum::Router;
use objection::{ObjectionService, Session};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
	let app = Router::new().route_service("/ui.ws", ObjectionService::<Instance>::new(()));

	let listener = TcpListener::bind(("localhost", 8000)).await.unwrap();
	println!("listening at ws://localhost:8000/ui.ws");

	axum::serve(listener, app).await.unwrap();
}

struct Instance;

impl Session for Instance {
	type Context = ();
	type PeerEvent = ();

	async fn create(_: Option<String>, _: &Self::Context, _: objection::Controller<'_>) -> Result<Self> {
		Ok(Instance)
	}

	async fn watch_object(&mut self, id: &str, _: &Self::Context, _: objection::Controller<'_>) -> Result<()> {
		println!("Watch object {id}");

		Ok(())
	}
}
