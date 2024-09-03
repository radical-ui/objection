use axum::{serve, Router};
use log::info;
use objection::{object_provider, Object, ObjectRouter, ObjectionService, Session, TabBar, Theme};
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() {
	env_logger::init();

	let mut ui = ObjectRouter::new();

	ui.object("home")
		.provider(object_provider!(user: &mut User, state => {
			let mut object = Object::default();

			object
				.set_title("Home")
				.set_subtitle("My home")
				.set_description("Foo")
				.set_icon("house");


			Ok(object)
		}))
		.commit();

	ui.object("about")
		.provider(object_provider!(user: &mut User, state => {
			let mut object = Object::default();

			object.set_title("About");

			Ok(object)
		}))
		.commit();

	let app = Router::new().route_service("/ui.ws", ObjectionService::new(ui, ()));
	let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
	info!("listening at http://localhost:8000");

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
		let mut theme = Theme::default();
		let mut tab_bar = TabBar::default();

		tab_bar.add_object("home");
		tab_bar.add_object("about");

		theme.set_tab_bar(tab_bar);

		theme
	}
}
