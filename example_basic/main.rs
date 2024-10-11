use axum::{serve, Router};
use log::info;
use objection::{object_provider, CallToAction, Color, Object, ObjectRouter, ObjectionService, Paragraph, Session, Surface, TabBar, Theme};
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
				.set_subtitle("My home is full of what I love, and I long to be there")
				.set_icon("Home")
				.push_content(CallToAction {
					title: "View Nested".into(),
					icon: Some("ArrowRight".into()),
					target_object: "nested".into(),
					surface: "primary".into()
				});


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

	ui.object("nested")
		.provider(object_provider!(user: &mut User, state => {
			let mut object = Object::default();

			object
				.set_title("Nested")
				.set_subtitle("This is a nested item here. There is nothing special about it.")
				.set_image("https://picsum.photos/1000")
				.push_content(Paragraph {
					text: "The placeholder text, beginning with the line “Lorem ipsum dolor sit amet, consectetur adipiscing elit”, looks like Latin because in its youth, centuries ago, it was Latin.".into()
				});

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

		tab_bar.add_object("home").add_object("about");

		theme
			.set_tab_bar(tab_bar)
			.set_default_uniform_surface(Surface {
				background_color_1: Color::rgb(37, 37, 37),
				background_color_2: Color::rgb(50, 50, 50),
				background_color_3: Color::rgb(60, 60, 60),
				background_color_4: Color::rgb(73, 73, 73),

				foreground_color_1: Color::rgb(255, 255, 255),
				foreground_color_2: Color::rgb(225, 225, 225),
				foreground_color_3: Color::rgb(200, 200, 200),
				foreground_color_4: Color::rgb(170, 170, 170),

				primary_color_1: Color::rgb(236, 213, 146),
				primary_color_2: Color::rgb(236, 213, 146).with_alpha(230),
				primary_color_3: Color::rgb(236, 213, 146).with_alpha(200),
				primary_color_4: Color::rgb(236, 213, 146).with_alpha(100),

				glow_color: None,
			})
			.set_uniform_surface(
				"primary",
				Surface {
					background_color_1: Color::rgb(236, 213, 146),
					background_color_2: Color::rgb(236, 213, 146),
					background_color_3: Color::rgb(236, 213, 146),
					background_color_4: Color::rgb(236, 213, 146),

					foreground_color_1: Color::rgb(255, 255, 255),
					foreground_color_2: Color::rgb(236, 213, 146),
					foreground_color_3: Color::rgb(236, 213, 146),
					foreground_color_4: Color::rgb(236, 213, 146),

					primary_color_1: Color::rgb(236, 213, 146),
					primary_color_2: Color::rgb(236, 213, 146),
					primary_color_3: Color::rgb(236, 213, 146),
					primary_color_4: Color::rgb(236, 213, 146),

					glow_color: None,
				},
			);

		theme
	}
}
