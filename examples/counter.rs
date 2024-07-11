use svelte_toolbox::{Application, CenterLayout, Color, Flex, FlexKind, HasActionKey, HasSession, IconButton, Theme, Ui, Window};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, HasActionKey, Serialize, Deserialize)]
enum CounterKey {
	Increment,
	Decrement,
}

struct Counter {
	count: i64,
}

impl HasSession for Counter {
	type State = ();

	async fn update(&mut self, state: Self::State, ui: &mut Ui) {
		match CounterKey::parse(&ui.take_action().unwrap()).unwrap() {
			CounterKey::Increment => self.count += 1,
			CounterKey::Decrement => self.count -= 1,
		};

		ui.refresh(self.get_window(state, ui))
	}

	fn get_window(&mut self, _: Self::State, _: &Ui) -> Window {
		Window {
			root_component: CenterLayout::new("A Counter")
				.subtitle(format!("Currently at: {}", self.count))
				.body(
					Flex::new(FlexKind::Row)
						.gap(5)
						.auto_item(IconButton::new("mdi-plus").action(CounterKey::Increment).size(40).color(Color::Primary(100)))
						.auto_item(IconButton::new("mdi-minus").action(CounterKey::Decrement).size(40).color(Color::Primary(100))),
				)
				.thin()
				.into(),
			theme: Some(Theme::default()),
			title: "Hello, World!".into(),
		}
	}

	async fn create(_: Self::State, _: Option<&Uuid>, _: &Ui) -> Counter {
		Counter { count: 0 }
	}
}

#[tokio::main]
async fn main() {
	env_logger::init();

	Application::<Counter>::new(()).listen(3000).await;
}
