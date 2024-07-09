use svelte_toolbox::{Application, CenterLayout, Flex, FlexKind, HasActionKey, HasSession, Label, TextInput, Theme, Ui, UpdateBoundary, UpdateHook, Window};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, HasActionKey, Serialize, Deserialize)]
enum HelloKey {
	Type,
}

struct Hello {
	name: String,
	update_hook: UpdateHook,
}

impl Hello {
	fn as_label(&self) -> Label {
		Label::new(format!("Hello, {}", self.name))
	}
}

impl HasSession for Hello {
	type State = ();

	async fn update(&mut self, _: (), ui: &mut Ui) {
		let action = ui.take_action().unwrap();

		match HelloKey::parse(&action).unwrap() {
			HelloKey::Type => {
				self.name = action.payload().unwrap().as_str().unwrap().into();
				self.update_hook.update(ui, self.as_label())
			}
		}
	}

	fn get_window(&mut self, _: (), _: &Ui) -> Window {
		Window {
			root_component: CenterLayout::new("Greetings!")
				.subtitle("Type in the field to see surgical updates below.")
				.body(
					Flex::new(FlexKind::Column)
						.gap(20)
						.auto_item(TextInput::new("Name").initial_value(&self.name).change_action(HelloKey::Type))
						.auto_item(UpdateBoundary::new(&self.update_hook).child(self.as_label())),
				)
				.thin()
				.into(),
			theme: Some(Theme::default()),
			title: "Hello, World!".into(),
		}
	}

	async fn create(_: (), _: Option<&Uuid>, _: &Ui) -> Hello {
		Hello {
			name: "World".into(),
			update_hook: UpdateHook::new(),
		}
	}
}

#[tokio::main]
async fn main() {
	env_logger::init();

	Application::<Hello>::new(()).listen(3000).await;
}
