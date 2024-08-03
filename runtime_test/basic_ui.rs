use objection::{IntoComponentIndex, Ui};
use objection_derive_event_symbol::EventSymbol;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bindings::*;

const TEST_TEXT: &str = include_str!("test_text.txt");

pub fn get_basic_ui(ui: Ui) -> Component {
	ScrollableBox::new()
		.body(
			Padding::new().all(30).body(
				Flex::new(FlexKind::Column)
					.gap(50)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("event_blocker", {
							#[derive(Debug, EventSymbol, Serialize, Deserialize)]
							pub enum Event {
								Foo,
							}
							EventBlocker::new(true).body(Button::new("Disabled").event(ui.scope(Event::Foo).event_key()))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("breadcrumbs", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Foo,
								Bar,
								Bin,
							}
							Breadcrumbs::new()
								.crumb(ui.scope(Event::Foo).event_key(), "Hi")
								.crumb(ui.scope(Event::Bar).event_key(), "Bye")
								.crumb(ui.scope(Event::Bin).event_key(), "Bock")
								.current("This")
								.body(Label::new("Some Body"))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("button", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Foo,
								Bar,
							}
							Flex::new(FlexKind::Column)
								.gap(10)
								.align(FlexAlign::Center)
								.justify(FlexJustify::Center)
								.item(
									FlexGrowth::Auto,
									Flex::new(FlexKind::Row)
										.gap(10)
										.align(FlexAlign::Center)
										.item(
											FlexGrowth::Auto,
											Button::new("Small Button").event(ui.scope(Event::Foo).event_key()).size(ButtonSize::Small),
										)
										.item(FlexGrowth::Auto, Button::new("Medium Button").event(ui.scope(Event::Foo).event_key()))
										.item(
											FlexGrowth::Auto,
											Button::new("Large Button").event(ui.scope(Event::Bar).event_key()).size(ButtonSize::Large),
										),
								)
								.item(
									FlexGrowth::Auto,
									Flex::new(FlexKind::Row)
										.gap(10)
										.item(
											FlexGrowth::Auto,
											Button::new("Fore Button").event(ui.scope(Event::Foo).event_key()).color(Color::Fore(5.0)),
										)
										.item(
											FlexGrowth::Auto,
											Button::new("Success Button")
												.event(ui.scope(Event::Foo).event_key())
												.color(Color::Success(100.0)),
										)
										.item(
											FlexGrowth::Auto,
											Button::new("Danger Button").event(ui.scope(Event::Foo).event_key()).color(Color::Danger(100.0)),
										),
								)
								.item(
									FlexGrowth::Auto,
									Flex::new(FlexKind::Row)
										.gap(10)
										.item(
											FlexGrowth::Auto,
											Button::new("Leading Icon")
												.event(ui.scope(Event::Foo).event_key())
												.leading_icon("mdi-ab-testing"),
										)
										.item(
											FlexGrowth::Auto,
											Button::new("Trailing Icon")
												.event(ui.scope(Event::Foo).event_key())
												.trailing_icon("mdi-ab-testing"),
										)
										.item(
											FlexGrowth::Auto,
											Button::new("Both")
												.event(ui.scope(Event::Bar).event_key())
												.trailing_icon("mdi-ab-testing")
												.leading_icon("mdi-ab-testing")
												.outline(),
										),
								)
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("card", Padding::new().all(10).body(Card::new().body(Label::new("Hey! I am a card!")))),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("card", {
							Padding::new()
								.all(10)
								.body(Card::new().body(Label::new("Hey! I am a red card!")).color(ColorType::Danger))
						}),
					)
					.item(FlexGrowth::Auto, PreviewBox::new("center", Center::new().body(Label::new("Hello, World!"))))
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("center_layout", {
							CenterLayout::new("Normal Center Layout")
								.subtitle("Some Subtitle")
								.body(Button::new("Hello there!").full())
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("center_layout", {
							CenterLayout::new("Thin Center Layout")
								.subtitle("Some Subtitle")
								.thin()
								.body(Button::new("Hello there!").full())
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("checkbox_input", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Foo,
							}

							Flex::new(FlexKind::Column)
								.item(
									FlexGrowth::Auto,
									CheckboxInput::new("Allow tracking").checked_if(true).event(ui.scope(Event::Foo).event_key()),
								)
								.item(FlexGrowth::Auto, CheckboxInput::new("Allow tracking (disabled)").checked_if(false))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("circle_progress", CircleProgress::new(Label::new("Hello"), 0.5)),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("divider", {
							Flex::new(FlexKind::Column)
								.gap(10)
								.item(FlexGrowth::Auto, Label::new("Slight"))
								.item(FlexGrowth::Auto, Divider::new().distinction(DividerDistinction::Slight))
								.item(FlexGrowth::Auto, Label::new("Medium"))
								.item(FlexGrowth::Auto, Divider::new().distinction(DividerDistinction::Medium))
								.item(FlexGrowth::Auto, Label::new("Profound"))
								.item(FlexGrowth::Auto, Divider::new().distinction(DividerDistinction::Profound))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("flex", {
							Flex::new(FlexKind::Row)
								.gap(10)
								.item(FlexGrowth::Expand, Card::new().body(Label::new("First")))
								.item(FlexGrowth::Expand, Card::new().body(Label::new("Second")))
						}),
					)
					.item(FlexGrowth::Auto, PreviewBox::new("fragment", Fragment::new()))
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("header", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							pub enum Event {
								Foo,
								Bar,
							}
							Flex::new(FlexKind::Column)
								.gap(30)
								.item(
									FlexGrowth::Auto,
									Header::new("With event Items")
										.subtitle("A subtitle here")
										.size(HeaderSize::Large)
										.event_item(ui.scope(Event::Foo).event_key(), "mdi-pencil", "Do Foo")
										.event_item(
											ui.scope(Event::Bar).event_key(),
											"mdi-ab-testing",
											"A very long comment that will take up some notable space",
										),
								)
								.item(
									FlexGrowth::Auto,
									Header::new("With event Items")
										.subtitle("A subtitle here")
										.size(HeaderSize::Medium)
										.event_item(ui.scope(Event::Foo).event_key(), "mdi-pencil", "Do Foo")
										.event_item(ui.scope(Event::Bar).event_key(), "mdi-ab-testing", "Do Bar"),
								)
								.item(
									FlexGrowth::Auto,
									Header::new("With event Items")
										.subtitle("A subtitle here")
										.title_edit_event(ui.scope(Event::Foo).event_key())
										.subtitle_edit_event(ui.scope(Event::Bar).event_key())
										.subtitle_placeholder("No description")
										.size(HeaderSize::Small)
										.event_item(ui.scope(Event::Foo).event_key(), "mdi-pencil", "Do Foo")
										.event_item(ui.scope(Event::Bar).event_key(), "mdi-ab-testing", "Do Bar"),
								)
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("icon", {
							Flex::new(FlexKind::Row)
								.gap(30)
								.justify(FlexJustify::Center)
								.align(FlexAlign::Center)
								.item(FlexGrowth::Auto, Icon::new("mdi-ab-testing").size(30).color(Color::Primary(100.0)))
								.item(FlexGrowth::Auto, Icon::new("mdi-account-arrow-left").size(30).color(Color::Success(100.0)))
								.item(FlexGrowth::Auto, Icon::new("mdi-access-point").size(30).color(Color::Danger(50.0)))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("icon_button", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Foo,
							}
							Flex::new(FlexKind::Row)
								.gap(20)
								.item(
									FlexGrowth::Auto,
									IconButton::new("mdi-ab-testing")
										.color(Color::Primary(100.0))
										.title("A description of what this does and it is a rather long description")
										.size(40)
										.event(ui.scope(Event::Foo).event_key()),
								)
								.item(FlexGrowth::Auto, IconButton::new("mdi-ab-testing"))
								.item(
									FlexGrowth::Auto,
									IconButton::new("mdi-ab-testing")
										.color(Color::Primary(100.0))
										.event(ui.scope(Event::Foo).event_key()),
								)
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("image", {
							Image::new("https://images.unsplash.com/photo-1711436470690-cf49602d1cf1")
								.width(300)
								.height(300)
								.fit(ImageFit::Cover)
								.decorate()
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("label", {
							#[derive(Serialize, Deserialize, EventSymbol)]
							enum Event {
								Foo,
							}
							Flex::new(FlexKind::Column)
								.gap(5)
								.justify(FlexJustify::Center)
								.align(FlexAlign::Center)
								.item(FlexGrowth::Auto, Label::new("Some Label"))
								.item(FlexGrowth::Auto, Label::new("Italic").italic())
								.item(FlexGrowth::Auto, Label::new("Bold").bold())
								.item(FlexGrowth::Auto, Label::new("Another Color").color(Color::Primary(100.0)))
								.item(
									FlexGrowth::Auto,
									Label::new("This one is editable")
										.edit_event(ui.scope(Event::Foo).event_key())
										.color(Color::Primary(100.0)),
								)
								.item(
									FlexGrowth::Auto,
									Flex::new(FlexKind::Row)
										.item(FlexGrowth::Auto, Label::new("And so is this").edit_event(ui.scope(Event::Foo).event_key()))
										.item(
											FlexGrowth::Auto,
											Label::new("And this too (with a placeholder)")
												.edit_event(ui.scope(Event::Foo).event_key())
												.placeholder("This is the placeholder!!!! It is pretty long."),
										),
								)
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("nested_flow", {
							NestedFlow::new()
								.indent()
								.item(NestedFlowItem::Header(NestedFlowHeaderItem::new("Created Alone")))
								.item(NestedFlowItem::Content(NestedFlowContentItem::new("Options", Label::new("Hi there!"))))
								.item(NestedFlowItem::Content(NestedFlowContentItem::new("Evil Plans", Label::new("Bad bad here"))))
								.item(NestedFlowItem::Content(NestedFlowContentItem::new("Good Plans", Label::new("Good good here!"))))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("nested_flow", {
							NestedFlow::new()
								.indent()
								.item(NestedFlowItem::Header(NestedFlowHeaderItem::new("Created Alone")))
								.item(NestedFlowItem::Content(NestedFlowContentItem::new("Options", Label::new("Hi there!"))))
								.item(NestedFlowItem::Content(NestedFlowContentItem::new("Evil Plans", Label::new("Bad bad here"))))
								.item(NestedFlowItem::Content(NestedFlowContentItem::new("Good Plans", Label::new("Good good here!"))))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("padding", Padding::new().all(30).body(Card::new().body(Label::new("See, it is padded!")))),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("radio_input", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Batter,
							}
							Flex::new(FlexKind::Column)
								.gap(30)
								.item(
									FlexGrowth::Auto,
									RadioInput::new().event(ui.scope(Event::Batter).event_key()).item(0, "Red").item(1, "Green"),
								)
								.item(
									FlexGrowth::Auto,
									RadioInput::new()
										.event(ui.scope(Event::Batter).event_key())
										.item(0, "Hi")
										.item_full(
											RadioItem::new(1, "Bye")
												.description(Label::new("This is greeting that people say when they are bidding farewell to a friend")),
										)
										.item_full(RadioItem::new(2, "Adieu").description(Label::new("The french form of \"Bye\""))),
								)
								.item(
									FlexGrowth::Auto,
									RadioInput::new()
										.item(0, "all are disabled here")
										.item_full(
											RadioItem::new(1, "Bye")
												.description(Label::new("This is greeting that people say when they are bidding farewell to a friend")),
										)
										.item_full(RadioItem::new(2, "Adieu").description(Label::new("The french form of \"Bye\""))),
								)
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("scrollable_box", ScrollableBox::new().body(Padding::new().all(20).body(Label::new(TEST_TEXT)))),
					)
					.item(FlexGrowth::Auto, PreviewBox::new("sidebar_layout", SidebarLayout::new("Abc Corp")))
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("sidebar_layout", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Foo,
								Bar,
							}

							SidebarLayout::new("Abc Corp")
								.title_event(ui.scope(Event::Foo).event_key())
								.logo_full(Image::new("https://github.githubassets.com/assets/3m-0151c2fda0ce.svg").width(30).height(30))
								.event_item_full(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(ui.scope(Event::Foo).event_key()))
								.event_item_full(SidebarItem::new("Activities").icon("mdi-ab-testing").event(ui.scope(Event::Bar).event_key()))
								.group_full(
									SidebarGroup::new("Main")
										.item_full(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(ui.scope(Event::Foo).event_key()))
										.item_full(SidebarItem::new("Activities").icon("mdi-ab-testing").event(ui.scope(Event::Bar).event_key())),
								)
								.group_full(
									SidebarGroup::new("Records")
										.item_full(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(ui.scope(Event::Foo).event_key()))
										.item_full(SidebarItem::new("Activities").icon("mdi-ab-testing").event(ui.scope(Event::Bar).event_key())),
								)
								.initial_event(ui.scope(Event::Foo).event_key())
								.footer(Center::new().body(Label::new("Za feetsies")))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("skeleton", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								Foo,
							}
							Padding::new().all(10).body(
								Flex::new(FlexKind::Column)
									.gap(10)
									.item(
										FlexGrowth::Auto,
										Skeleton::new()
											.linked_event_symbols(ui.scope(Event::Foo).event_key::<()>().get_dynamic_symbols())
											.body(
												Card::new().body(
													Flex::new(FlexKind::Column)
														.gap(10)
														.item(
															FlexGrowth::Auto,
															RadioInput::new()
																.item(0, "Hi")
																.item_full(RadioItem::new(1, "Bye").description(Label::new(
																	"This is greeting that people say when they are bidding farewell to a friend",
																)))
																.item_full(RadioItem::new(2, "Adieu").description(Label::new("The french form of \"Bye\""))),
														)
														.item(
															FlexGrowth::Auto,
															Image::new("https://images.unsplash.com/photo-1716369415085-4a6876f91840")
																.width(300)
																.height(200)
																.fit(ImageFit::Cover)
																.decorate(),
														),
												),
											),
									)
									.item(FlexGrowth::Auto, Button::new("Load").event(ui.scope(Event::Foo).event_key())),
							)
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("table", {
							Table::new()
								.column("Id")
								.column_full(TableColumn::new("Name").expand())
								.column("")
								.rows(Vec::from([
									Vec::<Component>::from([
										Label::new("82").into_index(),
										Label::new("Jason").into_index(),
										Button::new("View").size(ButtonSize::Small).into_index(),
									]),
									Vec::<Component>::from([
										Label::new("84").into_index(),
										Label::new("James").into_index(),
										Button::new("View").size(ButtonSize::Small).into_index(),
									]),
									Vec::<Component>::from([
										Label::new("103").into_index(),
										Label::new("Jeehoshofat Bartholemew, Duke of Northumberland, King of \"The Rose Garden\", the sixteenth").into_index(),
										Button::new("View").size(ButtonSize::Small).into_index(),
									]),
								]))
						}),
					)
					.item(
						FlexGrowth::Auto,
						PreviewBox::new("text_input", {
							#[derive(EventSymbol, Serialize, Deserialize)]
							enum Event {
								InputChanged,
								InputBlurred,
								OptionSelected,
								Submit,
							}

							ScrollableBox::new().body(
								Padding::new().all(30).body(
									Flex::new(FlexKind::Column)
										.gap(20)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Username")
												.change_event(ui.scope(Event::InputChanged).event_key())
												.submit_event(ui.scope(Event::Submit).event_key()),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Password")
												.role(TextInputRole::Password)
												.blur_event(ui.scope(Event::InputBlurred).event_key())
												.submit_event(ui.scope(Event::Submit).event_key()),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("With Initial Value")
												.initial_value("Hello there!")
												.blur_event(ui.scope(Event::InputBlurred).event_key())
												.submit_event(ui.scope(Event::Submit).event_key()),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Email (disabled)")
												.submit_event(ui.scope(Event::Submit).event_key())
												.role(TextInputRole::Email)
												.leading_icon("mdi-ab-testing"),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Dropdown with client filtering")
												.role(TextInputRole::Email)
												.blur_event(ui.scope(Event::InputBlurred).event_key())
												.submit_event(ui.scope(Event::Submit).event_key())
												.initial_dropdown_options(Vec::from([
													DropdownOption::new(Uuid::new_v4(), "Option 1"),
													DropdownOption::new(Uuid::new_v4(), "Option 2"),
													DropdownOption::new(Uuid::new_v4(), "Option 3"),
													DropdownOption::new(Uuid::new_v4(), "Option 4"),
													DropdownOption::new(Uuid::new_v4(), "Option 5"),
												])),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Dropdown with server filtering")
												.role(TextInputRole::Email)
												.change_event(ui.scope(Event::InputChanged).event_key())
												.submit_event(ui.scope(Event::Submit).event_key())
												.initial_dropdown_options(Vec::from([
													DropdownOption::new(Uuid::new_v4(), "Option 1"),
													DropdownOption::new(Uuid::new_v4(), "Option 2"),
													DropdownOption::new(Uuid::new_v4(), "Option 3").is_disabled(),
													DropdownOption::new(Uuid::new_v4(), "Option 4"),
													DropdownOption::new(Uuid::new_v4(), "Option 5"),
												])),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Dropdown without free text and client filtering")
												.role(TextInputRole::Email)
												.option_selected_event(ui.scope(Event::OptionSelected).event_key())
												.submit_event(ui.scope(Event::Submit).event_key())
												.initial_dropdown_options(Vec::from([
													DropdownOption::new(Uuid::new_v4(), "Option 1"),
													DropdownOption::new(Uuid::new_v4(), "Option 2"),
													DropdownOption::new(Uuid::new_v4(), "Option 3"),
													DropdownOption::new(Uuid::new_v4(), "Option 4"),
													DropdownOption::new(Uuid::new_v4(), "Option 5"),
												])),
										)
										.item(
											FlexGrowth::Auto,
											TextInput::new("Dropdown without free text and client filtering and multiple")
												.role(TextInputRole::Email)
												.option_selected_event(ui.scope(Event::OptionSelected).event_key())
												.submit_event(ui.scope(Event::Submit).event_key())
												.multiple()
												.initial_dropdown_options(Vec::from([
													DropdownOption::new(Uuid::new_v4(), "Option 1"),
													DropdownOption::new(Uuid::new_v4(), "Option 2"),
													DropdownOption::new(Uuid::new_v4(), "Option 3"),
													DropdownOption::new(Uuid::new_v4(), "Option 4"),
													DropdownOption::new(Uuid::new_v4(), "Option 5"),
												])),
										),
								),
							)
						}),
					),
			),
		)
		.into_index()
}
