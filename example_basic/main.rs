use axum::{serve, Router};
use objection::{ObjectRouter, ObjectionService, Session, Theme};
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() {
	let objects = ObjectRouter::<User>::new();

	let app = Router::new().route_service("/ui.ws", ObjectionService::new(objects, ()));
	let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

	serve(listener, app).await.unwrap();
}

struct User {}

impl Session for User {
	type Context = ();
	type PeerEvent = ();

	async fn create(id: &Uuid, context: Self::Context) -> Self {
		User {}
	}

	async fn get_theme(&mut self) -> Theme {
		Theme { tab_bar: None }
	}
}
