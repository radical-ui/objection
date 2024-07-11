use svelte_toolbox::{
	Application, Button, Center, CenterLayout, Flex, FlexKind, HasActionKey, HasSession, Modal, ModalSize, Theme, Ui, UpdateBoundary, UpdateHook, Window,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, HasActionKey)]
enum Event {
	Cancel,
	Finish,
	Open(ModalSize),
}

struct Modals {
	update_hook: UpdateHook,
}

impl HasSession for Modals {
	type State = ();

	async fn update(&mut self, _: Self::State, ui: &mut Ui) {
		match Event::parse(&ui.take_action().unwrap()).unwrap() {
			Event::Cancel => self.update_hook.update(ui, ()),
			Event::Finish => self.update_hook.update(ui, ()),
			Event::Open(size) => self.update_hook.update(
				ui,
				Modal::new("Some title")
					.description("Some description")
					.cancel_action("Custom Cancel", Event::Cancel)
					.finish_action("Custom Finish", Event::Finish)
					.size(size)
					.body(Center::new().body("Hello, World!")),
			),
		};
	}

	fn get_window(&mut self, _: Self::State, _: &Ui) -> Window {
		Window {
			root_component: CenterLayout::new("Some Modals")
				.body(
					Flex::new(FlexKind::Column)
						.auto_item(
							Flex::new(FlexKind::Row)
								.gap(10)
								.auto_item(Button::new("Small").action(Event::Open(ModalSize::Small)))
								.auto_item(Button::new("Medium").action(Event::Open(ModalSize::Medium)))
								.auto_item(Button::new("Large").action(Event::Open(ModalSize::Large))),
						)
						.auto_item(UpdateBoundary::new(&self.update_hook).child(())),
				)
				.into(),
			theme: Some(Theme::default()),
			title: "Hello, World!".into(),
		}
	}

	async fn create(_: Self::State, _: Option<&Uuid>, _: &Ui) -> Modals {
		Modals {
			update_hook: UpdateHook::new(),
		}
	}
}

#[tokio::main]
async fn main() {
	env_logger::init();

	Application::<Modals>::new(()).listen(3000).await;
}
