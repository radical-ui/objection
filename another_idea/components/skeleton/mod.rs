use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Component, HasActionKey};

/// TODO
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Event {
/// 	Foo
/// }
///
/// Padding::all(10)
/// 	.body(
/// 		Flex::new(FlexKind::Column)
/// 		.gap(10)
/// 		.auto_item(
/// 			Skeleton::new(
/// 				Event::Foo,
///					Card::new().body(
/// 					Flex::new(FlexKind::Column)
/// 						.gap(10)
/// 						.auto_item(
/// 							RadioInput::new()
/// 								.item(0, "Hi")
/// 								.described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend"))
/// 								.described_item(2, "Adieu", Label::new("The french form of \"Bye\""))
/// 						)
/// 						.auto_item(
/// 							Image::new("https://images.unsplash.com/photo-1716369415085-4a6876f91840?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxlZGl0b3JpYWwtZmVlZHwyfHx8ZW58MHx8fHx8")
/// 								.width(300)
/// 								.height(200)
/// 								.fit(ImageFit::Cover)
/// 								.decorate()
/// 						)
/// 				)
/// 			)
/// 		)
/// 		.auto_item(Button::new("Load").action(Event::Foo))
/// 	)
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Skeleton {
	linked_action: ActionKey,
	body: Box<Component>,
}

impl Skeleton {
	pub fn new(linked_action: impl HasActionKey, body: impl Into<Component>) -> Skeleton {
		Skeleton {
			linked_action: linked_action.get_action_key(),
			body: Box::new(body.into()),
		}
	}
}
