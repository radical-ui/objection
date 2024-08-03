impl objection::IntoComponentIndex for Divider {
    type Index = Component;
    fn into(self) -> Component {
        Component::Divider(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Card {
    type Index = Component;
    fn into(self) -> Component {
        Component::Card(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Label {
    type Index = Component;
    fn into(self) -> Component {
        Component::Label(Box::new(self))
    }
}
impl objection::IntoComponentIndex for NoticeManager {
    type Index = Component;
    fn into(self) -> Component {
        Component::NoticeManager(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Header {
    type Index = Component;
    fn into(self) -> Component {
        Component::Header(Box::new(self))
    }
}
impl objection::IntoComponentIndex for UpdateBoundary {
    type Index = Component;
    fn into(self) -> Component {
        Component::UpdateBoundary(Box::new(self))
    }
}
impl objection::IntoComponentIndex for EventBlocker {
    type Index = Component;
    fn into(self) -> Component {
        Component::EventBlocker(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Image {
    type Index = Component;
    fn into(self) -> Component {
        Component::Image(Box::new(self))
    }
}
impl objection::IntoComponentIndex for TextInput {
    type Index = Component;
    fn into(self) -> Component {
        Component::TextInput(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Skeleton {
    type Index = Component;
    fn into(self) -> Component {
        Component::Skeleton(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Flex {
    type Index = Component;
    fn into(self) -> Component {
        Component::Flex(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Modal {
    type Index = Component;
    fn into(self) -> Component {
        Component::Modal(Box::new(self))
    }
}
impl objection::IntoComponentIndex for IconButton {
    type Index = Component;
    fn into(self) -> Component {
        Component::IconButton(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Button {
    type Index = Component;
    fn into(self) -> Component {
        Component::Button(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Table {
    type Index = Component;
    fn into(self) -> Component {
        Component::Table(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Center {
    type Index = Component;
    fn into(self) -> Component {
        Component::Center(Box::new(self))
    }
}
impl objection::IntoComponentIndex for CircleProgress {
    type Index = Component;
    fn into(self) -> Component {
        Component::CircleProgress(Box::new(self))
    }
}
impl objection::IntoComponentIndex for CheckboxInput {
    type Index = Component;
    fn into(self) -> Component {
        Component::CheckboxInput(Box::new(self))
    }
}
impl objection::IntoComponentIndex for ScrollableBox {
    type Index = Component;
    fn into(self) -> Component {
        Component::ScrollableBox(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Breadcrumbs {
    type Index = Component;
    fn into(self) -> Component {
        Component::Breadcrumbs(Box::new(self))
    }
}
impl objection::IntoComponentIndex for TitleSegment {
    type Index = Component;
    fn into(self) -> Component {
        Component::TitleSegment(Box::new(self))
    }
}
impl objection::IntoComponentIndex for RadioInput {
    type Index = Component;
    fn into(self) -> Component {
        Component::RadioInput(Box::new(self))
    }
}
impl objection::IntoComponentIndex for NestedFlow {
    type Index = Component;
    fn into(self) -> Component {
        Component::NestedFlow(Box::new(self))
    }
}
impl objection::IntoComponentIndex for PreviewBox {
    type Index = Component;
    fn into(self) -> Component {
        Component::PreviewBox(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Padding {
    type Index = Component;
    fn into(self) -> Component {
        Component::Padding(Box::new(self))
    }
}
impl objection::IntoComponentIndex for SidebarLayout {
    type Index = Component;
    fn into(self) -> Component {
        Component::SidebarLayout(Box::new(self))
    }
}
impl objection::IntoComponentIndex for CenterLayout {
    type Index = Component;
    fn into(self) -> Component {
        Component::CenterLayout(Box::new(self))
    }
}
impl objection::IntoComponentIndex for ThemeManager {
    type Index = Component;
    fn into(self) -> Component {
        Component::ThemeManager(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Icon {
    type Index = Component;
    fn into(self) -> Component {
        Component::Icon(Box::new(self))
    }
}
impl objection::IntoComponentIndex for Fragment {
    type Index = Component;
    fn into(self) -> Component {
        Component::Fragment(Box::new(self))
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "def")]
pub enum Component {
    /**A visual divider, which can be horizontal or vertical, and can have varying distinction.

**Example**

```rust Flex::new(FlexKind::Column) .gap(10) .auto_item("Slight") .auto_item(Divider::new().distinction(DividerDistinction::Slight)) .auto_item("Medium") .auto_item(Divider::new().distinction(DividerDistinction::Medium)) .auto_item("Profound") .auto_item(Divider::new().distinction(DividerDistinction::Profound)) ```
*/
    Divider(Box<Divider>),
    /**A card that can optionally be colored.

**Examples**

```rust Padding::all(10).body(Card::new().body(Label::new("Hey! I am a card!"))) ```

```rust Padding::all(10).body(Card::new().body(Label::new("Hey! I am a red card!")).color(ColorType::Danger)) ```
*/
    Card(Box<Card>),
    /**A simple label

**Example**

```rust #[derive(Serialize, Deserialize, HasActionKey)] enum Event { Foo }

Flex::new(FlexKind::Column) .gap(5) .justify(FlexJustify::Center) .align(FlexAlign::Center) .auto_item(Label::new("Some Label")) .auto_item(Label::new("Italic").italic()) .auto_item(Label::new("Bold").bold()) .auto_item(Label::new("Another Color").color(Color::Primary(100))) .auto_item(Label::new("This one is editable").edit_event(Event::Foo).color(Color::Primary(100))) .auto_item( Flex::new(FlexKind::Row) .auto_item(Label::new("And so is this").edit_event(Event::Foo)) .auto_item(Label::new("And this too (with a placeholder)").edit_event(Event::Foo).placeholder("This is the placeholder!!!! It is pretty long.")) ) ```
*/
    Label(Box<Label>),
    /**A notice manager for displaying notices on the screen
*/
    NoticeManager(Box<NoticeManager>),
    /**A simple page layout, with a title, subtitle, some possible event items, and a body. Additionally, a logo can appear off to the right.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] pub enum Event { Foo, Bar, }

Flex::new(FlexKind::Column) .gap(30) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Large) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "A very long comment that will take up some notable space") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Medium) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .title_edit_event(Event::Foo) .subtitle_edit_event(Event::Bar) .subtitle_placeholder("No description") .size(HeaderSize::Small) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) ```
*/
    Header(Box<Header>),
    /**A boundary that allows it's children to be updated without beaming down an entirely new widget tree.

You can optionally set a child to be displayed until an update is sent. Once an update has been sent has been sent, the original child will never be rendered. ```
*/
    UpdateBoundary(Box<UpdateBoundary>),
    /**TODO

**Example**

```rust #[derive(Debug, HasActionKey, Serialize, Deserialize)] pub enum Event { Foo }

ActionBlocker::new().body(Button::new("Disabled").event(Event::Foo)) ```
*/
    EventBlocker(Box<EventBlocker>),
    /**TODO

**Example**

```rust Image::new("https://images.unsplash.com/photo-1711436470690-cf49602d1cf1?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D") .width(300) .height(300) .fit(ImageFit::Cover) .decorate() ```
*/
    Image(Box<Image>),
    /**A text input. If no change event, blur event, or dropdown selection event is supplied, the input will be disabled.

If some initial dropdown options are supplied, but no `change_event` is supplied, the dropdown options will be sorted locally. If a `change_event` is supplied, the server is expected to send down a new list of dropdown options.

If no `option_selection_event` is supplied, the selected dropdown options will simply replace the input value, triggering the default value update behavior.

`allow_multiple_options` has no effect if an `option_selected_option` is not supplied. If it is, more that one option can be selected.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { InputChanged, InputBlurred, OptionSelected, Submit }

Padding::all(30).body( Flex::new(FlexKind::Column) .gap(20) .auto_item(TextInput::new("Username").change_event(Event::InputChanged).submit_event(Event::Submit)) .auto_item(TextInput::new("Password").role(TextInputRole::Password).blur_event(Event::InputBlurred).submit_event(Event::Submit)) .auto_item(TextInput::new("With Initial Value").initial_value("Hello there!").blur_event(Event::InputBlurred).submit_event(Event::Submit)) .auto_item(TextInput::new("Email (disabled)").submit_event(Event::Submit).role(TextInputRole::Email).leading_icon("mdi-ab-testing")) .auto_item( TextInput::new("Dropdown with client filtering") .role(TextInputRole::Email) .blur_event(Event::InputBlurred) .submit_event(Event::Submit) .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3"), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) .auto_item( TextInput::new("Dropdown with server filtering") .role(TextInputRole::Email) .change_event(Event::InputChanged) .submit_event(Event::Submit) .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3").is_disabled(), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) .auto_item( TextInput::new("Dropdown without free text and client filtering") .role(TextInputRole::Email) .option_selected_event(Event::OptionSelected) .submit_event(Event::Submit) .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3"), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) .auto_item( TextInput::new("Dropdown without free text and client filtering and multiple") .role(TextInputRole::Email) .option_selected_event(Event::OptionSelected) .submit_event(Event::Submit) .multiple() .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3"), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) ) ```
*/
    TextInput(Box<TextInput>),
    /**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo }

Padding::all(10) .body( Flex::new(FlexKind::Column) .gap(10) .auto_item( Skeleton::new( Event::Foo, Card::new().body( Flex::new(FlexKind::Column) .gap(10) .auto_item( RadioInput::new() .item(0, "Hi") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) .auto_item( Image::new("https://images.unsplash.com/photo-1716369415085-4a6876f91840?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxlZGl0b3JpYWwtZmVlZHwyfHx8ZW58MHx8fHx8") .width(300) .height(200) .fit(ImageFit::Cover) .decorate() ) ) ) ) .auto_item(Button::new("Load").event(Event::Foo)) ) ```
*/
    Skeleton(Box<Skeleton>),
    Flex(Box<Flex>),
    /**A modal that appears over all existing content, using the context from where it is placed.
*/
    Modal(Box<Modal>),
    /**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo }

Flex::new(FlexKind::Row) .gap(20) .auto_item( IconButton::new("mdi-ab-testing") .color(Color::Primary(100)) .title("A description of what this does and it is a rather long description") .size(40) .event(Event::Foo) ) .auto_item(IconButton::new("mdi-ab-testing")) .auto_item( IconButton::new("mdi-ab-testing") .color(Color::Primary(100)) .event(Event::Foo) ) ```
*/
    IconButton(Box<IconButton>),
    /**A button that has a label and an event.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, }

Flex::new(FlexKind::Column) .gap(10) .align(FlexAlign::Center) .justify(FlexJustify::Center) .auto_item( Flex::new(FlexKind::Row) .gap(10) .align(FlexAlign::Center) .auto_item( Button::new("Small Button") .event(Event::Foo) .size(ButtonSize::Small) ) .auto_item( Button::new("Medium Button") .event(Event::Foo) ) .auto_item( Button::new("Large Button") .event(Event::Bar) .size(ButtonSize::Large) ) ) .auto_item( Flex::new(FlexKind::Row) .gap(10) .auto_item( Button::new("Fore Button") .event(Event::Foo) .color(Color::Fore(5)) ) .auto_item( Button::new("Success Button") .event(Event::Foo) .color(Color::Success(100)) ) .auto_item( Button::new("Danger Button") .event(Event::Foo) .color(Color::Danger(100)) ) ) .auto_item( Flex::new(FlexKind::Row) .gap(10) .auto_item( Button::new("Leading Icon") .event(Event::Foo) .leading_icon("mdi-ab-testing") ) .auto_item( Button::new("Trailing Icon") .event(Event::Foo) .trailing_icon("mdi-ab-testing") ) .auto_item( Button::new("Both") .event(Event::Bar) .trailing_icon("mdi-ab-testing") .leading_icon("mdi-ab-testing") .outline() ) ) ```
*/
    Button(Box<Button>),
    /**TODO

**Example**

```rust Table::new() .column("Id") .expanded_column("Name") .column("") .rows(Vec::from([ Vec::<Component>::from([ Label::new("82").into(), Label::new("Jason").into(), Button::new("View").size(ButtonSize::Small).into() ]), Vec::<Component>::from([ Label::new("84").into(), Label::new("James").into(), Button::new("View").size(ButtonSize::Small).into() ]), Vec::<Component>::from([ Label::new("103").into(), Label::new("Jeehoshofat Bartholemew, Duke of Northumberland, King of \"The Rose Garden\", the sixteenth").into(), Button::new("View").size(ButtonSize::Small).into() ]), ])) ```
*/
    Table(Box<Table>),
    /**TODO

**Example**

```rust Center::new().body(Label::new("Hello, World!")) ```
*/
    Center(Box<Center>),
    /**TODO

**Example**

```rust CircleProgress::new() .value(0.5) .label("Hello") ```
*/
    CircleProgress(Box<CircleProgress>),
    /**A checkbox input, which can be either on or off.

At some point, this component should be combined with a sort of shared context on the frontend to connect with other checkboxes, define roots, and be in an intermediate state.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Action { Foo }

Flex::new(FlexKind::Column) .auto_item(CheckboxInput::new("Allow tracking").initial_value(true).event(Action::Foo)) .auto_item(CheckboxInput::new("Allow tracking (disabled)").initial_value(false)) ```
*/
    CheckboxInput(Box<CheckboxInput>),
    /**A scrollable box.

**Example**

```rust ScrollableBox::new().body(Padding::all(20).body("Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of \"de Finibus Bonorum et Malorum\" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, \"Lorem ipsum dolor sit amet..\", comes from a line in section 1.10.32. But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.")) ```
*/
    ScrollableBox(Box<ScrollableBox>),
    /**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, Bin, }

Breadcrumbs::new() .crumb(Event::Foo, "Hi") .crumb(Event::Bar, "Bye") .crumb(Event::Bin, "Bock") .current("This") .body("Some Body") ```
*/
    Breadcrumbs(Box<Breadcrumbs>),
    /**A component for setting the a title segment. Will be joined to any title segments in the component hierarchy using a '|'
*/
    TitleSegment(Box<TitleSegment>),
    /**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Batter, }

Flex::new(FlexKind::Column) .gap(30) .auto_item( RadioInput::new() .event(Event::Batter) .item(0, "Red") .item(1, "Green") ) .auto_item( RadioInput::new() .event(Event::Batter) .item(0, "Hi") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) .auto_item( RadioInput::new() .item(0, "all are disabled here") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) ```
*/
    RadioInput(Box<RadioInput>),
    /**TODO

**Indented Example**

```rust NestedFlow::new() .indent() .header("Created Alone") .content("Options", Label::new("Hi there!")) .content("Evil Plans", Label::new("Bad bad here")) .content("Good Plans", Label::new("Good good here!")) ```

**Not Indented Example**

```rust NestedFlow::new() .header("Created Alone") .content("Options", Label::new("Hi there!")) .content("Evil Plans", Label::new("Bad bad here")) .content("Good Plans", Label::new("Good good here!")) ```
*/
    NestedFlow(Box<NestedFlow>),
    /**A ui-decorated box for displaying content
*/
    PreviewBox(Box<PreviewBox>),
    /**A container with padding.

**Example**

```rust Padding::all(30).body(Card::new().body(Label::new("See, it is padded!"))) ```
*/
    Padding(Box<Padding>),
    /**A sidebar application layout.

**Example**

```rust SidebarLayout::new("Abc Corp") ```

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Action { Foo, Bar, }

SidebarLayout::new("Abc Corp") .title_event(Action::Foo) .logo(Image::new("https://github.githubassets.com/assets/3m-0151c2fda0ce.svg").width(30).height(30)) .event_item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .event_item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) .group( SidebarGroup::new("Main") .item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) ) .group( SidebarGroup::new("Records") .item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) ) .initial_event(Action::Foo) .footer(Center::new().body("Za feetsies")) ```
*/
    SidebarLayout(Box<SidebarLayout>),
    /**TODO

**Example**

```rust CenterLayout::new("Normal Center Layout").subtitle("Some Subtitle").body(Button::new("Hello there!").full()) ```

```rust CenterLayout::new("Thin Center Layout").subtitle("Some Subtitle").thin().body(Button::new("Hello there!").full()) ```
*/
    CenterLayout(Box<CenterLayout>),
    /**A theme manager for all components. Currently the theme of all components, regardless of location are affected, but this is expected to change
to where only child components are affected.
*/
    ThemeManager(Box<ThemeManager>),
    /**TODO

**Example**

```rust Flex::new(FlexKind::Row) .gap(30) .justify(FlexJustify::Center) .align(FlexAlign::Center) .auto_item( Icon::new("mdi-ab-testing", 30).color(Color::Primary(100))) .auto_item( Icon::new("mdi-account-arrow-left", 30).color(Color::Success(100))) .auto_item( Icon::new("mdi-access-point", 30).color(Color::Danger(50))) ```
*/
    Icon(Box<Icon>),
    ///A "nothing" component. Renders nothing.
    Fragment(Box<Fragment>),
}
impl objection::ComponentIndex for Component {
    fn to_value(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
/**An event that could be triggered, but that is not linked to any payload. This should only be used in cases where the event is not
actually triggered, but there is some reason to keep a reference to it.*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyEvent {
    pub action_path: Vec<String>,
    pub debug_symbol: Option<String>,
}
#[allow(dead_code)]
impl AnyEvent {
    ///Construct a new AnyEvent.
    pub fn new(action_path: Vec<String>) -> AnyEvent {
        AnyEvent {
            action_path,
            debug_symbol: None,
        }
    }
    pub fn action_path(mut self, action_path: Vec<String>) -> AnyEvent {
        self.action_path = action_path;
        self
    }
    pub fn debug_symbol(mut self, debug_symbol: impl Into<String>) -> AnyEvent {
        self.debug_symbol = Some(debug_symbol.into());
        self
    }
}
/**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, Bin, }

Breadcrumbs::new() .crumb(Event::Foo, "Hi") .crumb(Event::Bar, "Bye") .crumb(Event::Bin, "Bock") .current("This") .body("Some Body") ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Breadcrumbs {
    pub body: Option<Component>,
    pub crumbs: Vec<Crumb>,
    pub current: Option<String>,
}
#[allow(dead_code)]
impl Breadcrumbs {
    ///Construct a new Breadcrumbs.
    pub fn new(crumbs: Vec<Crumb>) -> Breadcrumbs {
        Breadcrumbs {
            body: None,
            crumbs,
            current: None,
        }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> Breadcrumbs {
        self.body = Some(body.into());
        self
    }
    pub fn crumbs(mut self, crumbs: Vec<Crumb>) -> Breadcrumbs {
        self.crumbs = crumbs;
        self
    }
    pub fn current(mut self, current: impl Into<String>) -> Breadcrumbs {
        self.current = Some(current.into());
        self
    }
}
/**A button that has a label and an event.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, }

Flex::new(FlexKind::Column) .gap(10) .align(FlexAlign::Center) .justify(FlexJustify::Center) .auto_item( Flex::new(FlexKind::Row) .gap(10) .align(FlexAlign::Center) .auto_item( Button::new("Small Button") .event(Event::Foo) .size(ButtonSize::Small) ) .auto_item( Button::new("Medium Button") .event(Event::Foo) ) .auto_item( Button::new("Large Button") .event(Event::Bar) .size(ButtonSize::Large) ) ) .auto_item( Flex::new(FlexKind::Row) .gap(10) .auto_item( Button::new("Fore Button") .event(Event::Foo) .color(Color::Fore(5)) ) .auto_item( Button::new("Success Button") .event(Event::Foo) .color(Color::Success(100)) ) .auto_item( Button::new("Danger Button") .event(Event::Foo) .color(Color::Danger(100)) ) ) .auto_item( Flex::new(FlexKind::Row) .gap(10) .auto_item( Button::new("Leading Icon") .event(Event::Foo) .leading_icon("mdi-ab-testing") ) .auto_item( Button::new("Trailing Icon") .event(Event::Foo) .trailing_icon("mdi-ab-testing") ) .auto_item( Button::new("Both") .event(Event::Bar) .trailing_icon("mdi-ab-testing") .leading_icon("mdi-ab-testing") .outline() ) ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Button {
    pub event: Option<objection::EventKey<()>>,
    pub color: Color,
    pub full: bool,
    pub label: String,
    pub leading_icon: Option<IconName>,
    pub outline: bool,
    pub size: ButtonSize,
    pub trailing_icon: Option<IconName>,
}
#[allow(dead_code)]
impl Button {
    pub fn event(mut self, event: objection::EventKey<()>) -> Button {
        self.event = Some(event);
        self
    }
    pub fn color(mut self, color_opacity: f64, color_kind: ColorType) -> Button {
        self.color = Color {
            opacity: color_opacity,
            kind: color_kind,
        };
        self
    }
    pub fn color_full(mut self, color: Color) -> Button {
        self.color = color;
        self
    }
    pub fn full(mut self) -> Button {
        self.full = true;
        self
    }
    pub fn full_if(mut self, full: bool) -> Button {
        self.full = full;
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> Button {
        self.label = label.into();
        self
    }
    pub fn leading_icon(mut self, leading_icon: IconName) -> Button {
        self.leading_icon = Some(leading_icon);
        self
    }
    pub fn outline(mut self) -> Button {
        self.outline = true;
        self
    }
    pub fn outline_if(mut self, outline: bool) -> Button {
        self.outline = outline;
        self
    }
    pub fn size(mut self, size: ButtonSize) -> Button {
        self.size = size;
        self
    }
    pub fn trailing_icon(mut self, trailing_icon: IconName) -> Button {
        self.trailing_icon = Some(trailing_icon);
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}
/**A card that can optionally be colored.

**Examples**

```rust Padding::all(10).body(Card::new().body(Label::new("Hey! I am a card!"))) ```

```rust Padding::all(10).body(Card::new().body(Label::new("Hey! I am a red card!")).color(ColorType::Danger)) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub body: Option<Component>,
    pub color: ColorType,
}
#[allow(dead_code)]
impl Card {
    ///Construct a new Card.
    pub fn new(color: ColorType) -> Card {
        Card { body: None, color }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> Card {
        self.body = Some(body.into());
        self
    }
    pub fn color(mut self, color: ColorType) -> Card {
        self.color = color;
        self
    }
}
/**TODO

**Example**

```rust Center::new().body(Label::new("Hello, World!")) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Center {
    pub body: Option<Component>,
}
#[allow(dead_code)]
impl Center {
    ///Construct a new Center.
    pub fn new() -> Center {
        Center { body: None }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> Center {
        self.body = Some(body.into());
        self
    }
}
/**TODO

**Example**

```rust CenterLayout::new("Normal Center Layout").subtitle("Some Subtitle").body(Button::new("Hello there!").full()) ```

```rust CenterLayout::new("Thin Center Layout").subtitle("Some Subtitle").thin().body(Button::new("Hello there!").full()) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CenterLayout {
    pub body: Option<Component>,
    pub subtitle: Option<String>,
    pub thin: bool,
    pub title: String,
}
#[allow(dead_code)]
impl CenterLayout {
    ///Construct a new CenterLayout.
    pub fn new(thin: bool, title: impl Into<String>) -> CenterLayout {
        CenterLayout {
            body: None,
            subtitle: None,
            thin,
            title: title.into(),
        }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> CenterLayout {
        self.body = Some(body.into());
        self
    }
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> CenterLayout {
        self.subtitle = Some(subtitle.into());
        self
    }
    pub fn thin(mut self) -> CenterLayout {
        self.thin = true;
        self
    }
    pub fn thin_if(mut self, thin: bool) -> CenterLayout {
        self.thin = thin;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> CenterLayout {
        self.title = title.into();
        self
    }
}
/**A checkbox input, which can be either on or off.

At some point, this component should be combined with a sort of shared context on the frontend to connect with other checkboxes, define roots, and be in an intermediate state.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Action { Foo }

Flex::new(FlexKind::Column) .auto_item(CheckboxInput::new("Allow tracking").initial_value(true).event(Action::Foo)) .auto_item(CheckboxInput::new("Allow tracking (disabled)").initial_value(false)) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckboxInput {
    pub event: Option<objection::EventKey<bool>>,
    pub initial_value: bool,
    pub label: String,
}
#[allow(dead_code)]
impl CheckboxInput {
    ///Construct a new CheckboxInput.
    pub fn new(initial_value: bool, label: impl Into<String>) -> CheckboxInput {
        CheckboxInput {
            event: None,
            initial_value,
            label: label.into(),
        }
    }
    pub fn event(mut self, event: objection::EventKey<bool>) -> CheckboxInput {
        self.event = Some(event);
        self
    }
    pub fn initial_value(mut self) -> CheckboxInput {
        self.initial_value = true;
        self
    }
    pub fn initial_value_if(mut self, initial_value: bool) -> CheckboxInput {
        self.initial_value = initial_value;
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> CheckboxInput {
        self.label = label.into();
        self
    }
}
/**TODO

**Example**

```rust CircleProgress::new() .value(0.5) .label("Hello") ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircleProgress {
    pub label: Label,
    pub size: f64,
    pub value: f64,
}
#[allow(dead_code)]
impl CircleProgress {
    ///Construct a new CircleProgress.
    pub fn new(label: Label, size: f64, value: f64) -> CircleProgress {
        CircleProgress {
            label,
            size,
            value,
        }
    }
    pub fn label(mut self) -> CircleProgress {
        self.label = Label {
            color: None,
            edit_event: None,
            is_bold: None,
            is_italic: None,
            placeholder: None,
            text: None,
        };
        self
    }
    pub fn label_full(mut self, label: Label) -> CircleProgress {
        self.label = label;
        self
    }
    pub fn size(mut self, size: f64) -> CircleProgress {
        self.size = size;
        self
    }
    pub fn value(mut self, value: f64) -> CircleProgress {
        self.value = value;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub opacity: f64,
    pub kind: ColorType,
}
#[allow(dead_code)]
impl Color {
    ///Construct a new Color.
    pub fn new(opacity: f64, kind: ColorType) -> Color {
        Color { opacity, kind }
    }
    pub fn opacity(mut self, opacity: f64) -> Color {
        self.opacity = opacity;
        self
    }
    pub fn kind(mut self, kind: ColorType) -> Color {
        self.kind = kind;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorDefinition {
    pub blue: f64,
    pub green: f64,
    pub red: f64,
}
#[allow(dead_code)]
impl ColorDefinition {
    ///Construct a new ColorDefinition.
    pub fn new(blue: f64, green: f64, red: f64) -> ColorDefinition {
        ColorDefinition {
            blue,
            green,
            red,
        }
    }
    pub fn blue(mut self, blue: f64) -> ColorDefinition {
        self.blue = blue;
        self
    }
    pub fn green(mut self, green: f64) -> ColorDefinition {
        self.green = green;
        self
    }
    pub fn red(mut self, red: f64) -> ColorDefinition {
        self.red = red;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorPalette {
    pub base: ColorDefinition,
    pub danger: ColorDefinition,
    pub decoration_fore: ColorDefinition,
    pub fore: ColorDefinition,
    pub notice: ColorDefinition,
    pub primary: ColorDefinition,
    pub secondary: ColorDefinition,
    pub success: ColorDefinition,
    pub warn: ColorDefinition,
}
#[allow(dead_code)]
impl ColorPalette {
    pub fn base(
        mut self,
        base_blue: f64,
        base_green: f64,
        base_red: f64,
    ) -> ColorPalette {
        self.base = ColorDefinition {
            blue: base_blue,
            green: base_green,
            red: base_red,
        };
        self
    }
    pub fn base_full(mut self, base: ColorDefinition) -> ColorPalette {
        self.base = base;
        self
    }
    pub fn danger(
        mut self,
        danger_blue: f64,
        danger_green: f64,
        danger_red: f64,
    ) -> ColorPalette {
        self.danger = ColorDefinition {
            blue: danger_blue,
            green: danger_green,
            red: danger_red,
        };
        self
    }
    pub fn danger_full(mut self, danger: ColorDefinition) -> ColorPalette {
        self.danger = danger;
        self
    }
    pub fn decoration_fore(
        mut self,
        decoration_fore_blue: f64,
        decoration_fore_green: f64,
        decoration_fore_red: f64,
    ) -> ColorPalette {
        self.decoration_fore = ColorDefinition {
            blue: decoration_fore_blue,
            green: decoration_fore_green,
            red: decoration_fore_red,
        };
        self
    }
    pub fn decoration_fore_full(
        mut self,
        decoration_fore: ColorDefinition,
    ) -> ColorPalette {
        self.decoration_fore = decoration_fore;
        self
    }
    pub fn fore(
        mut self,
        fore_blue: f64,
        fore_green: f64,
        fore_red: f64,
    ) -> ColorPalette {
        self.fore = ColorDefinition {
            blue: fore_blue,
            green: fore_green,
            red: fore_red,
        };
        self
    }
    pub fn fore_full(mut self, fore: ColorDefinition) -> ColorPalette {
        self.fore = fore;
        self
    }
    pub fn notice(
        mut self,
        notice_blue: f64,
        notice_green: f64,
        notice_red: f64,
    ) -> ColorPalette {
        self.notice = ColorDefinition {
            blue: notice_blue,
            green: notice_green,
            red: notice_red,
        };
        self
    }
    pub fn notice_full(mut self, notice: ColorDefinition) -> ColorPalette {
        self.notice = notice;
        self
    }
    pub fn primary(
        mut self,
        primary_blue: f64,
        primary_green: f64,
        primary_red: f64,
    ) -> ColorPalette {
        self.primary = ColorDefinition {
            blue: primary_blue,
            green: primary_green,
            red: primary_red,
        };
        self
    }
    pub fn primary_full(mut self, primary: ColorDefinition) -> ColorPalette {
        self.primary = primary;
        self
    }
    pub fn secondary(
        mut self,
        secondary_blue: f64,
        secondary_green: f64,
        secondary_red: f64,
    ) -> ColorPalette {
        self.secondary = ColorDefinition {
            blue: secondary_blue,
            green: secondary_green,
            red: secondary_red,
        };
        self
    }
    pub fn secondary_full(mut self, secondary: ColorDefinition) -> ColorPalette {
        self.secondary = secondary;
        self
    }
    pub fn success(
        mut self,
        success_blue: f64,
        success_green: f64,
        success_red: f64,
    ) -> ColorPalette {
        self.success = ColorDefinition {
            blue: success_blue,
            green: success_green,
            red: success_red,
        };
        self
    }
    pub fn success_full(mut self, success: ColorDefinition) -> ColorPalette {
        self.success = success;
        self
    }
    pub fn warn(
        mut self,
        warn_blue: f64,
        warn_green: f64,
        warn_red: f64,
    ) -> ColorPalette {
        self.warn = ColorDefinition {
            blue: warn_blue,
            green: warn_green,
            red: warn_red,
        };
        self
    }
    pub fn warn_full(mut self, warn: ColorDefinition) -> ColorPalette {
        self.warn = warn;
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ColorType {
    Primary,
    Fore,
    DecorationFore,
    Base,
    Danger,
    Warn,
    Success,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crumb {
    pub event: objection::EventKey<()>,
    pub text: String,
}
#[allow(dead_code)]
impl Crumb {
    ///Construct a new Crumb.
    pub fn new(event: objection::EventKey<()>, text: impl Into<String>) -> Crumb {
        Crumb { event, text: text.into() }
    }
    pub fn event(mut self, event: objection::EventKey<()>) -> Crumb {
        self.event = event;
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Crumb {
        self.text = text.into();
        self
    }
}
/**A visual divider, which can be horizontal or vertical, and can have varying distinction.

**Example**

```rust Flex::new(FlexKind::Column) .gap(10) .auto_item("Slight") .auto_item(Divider::new().distinction(DividerDistinction::Slight)) .auto_item("Medium") .auto_item(Divider::new().distinction(DividerDistinction::Medium)) .auto_item("Profound") .auto_item(Divider::new().distinction(DividerDistinction::Profound)) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Divider {
    pub direction: DividerDirection,
    pub distinction: DividerDistinction,
}
#[allow(dead_code)]
impl Divider {
    ///Construct a new Divider.
    pub fn new(direction: DividerDirection, distinction: DividerDistinction) -> Divider {
        Divider { direction, distinction }
    }
    pub fn direction(mut self, direction: DividerDirection) -> Divider {
        self.direction = direction;
        self
    }
    pub fn distinction(mut self, distinction: DividerDistinction) -> Divider {
        self.distinction = distinction;
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum DividerDirection {
    Horizontal,
    Vertical,
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum DividerDistinction {
    Profound,
    Medium,
    Slight,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropdownOption {
    pub description: Option<String>,
    pub id: String,
    pub informative: Option<String>,
    pub is_disabled: bool,
    pub title: String,
}
#[allow(dead_code)]
impl DropdownOption {
    ///Construct a new DropdownOption.
    pub fn new(
        id: impl Into<String>,
        is_disabled: bool,
        title: impl Into<String>,
    ) -> DropdownOption {
        DropdownOption {
            description: None,
            id: id.into(),
            informative: None,
            is_disabled,
            title: title.into(),
        }
    }
    pub fn description(mut self, description: impl Into<String>) -> DropdownOption {
        self.description = Some(description.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> DropdownOption {
        self.id = id.into();
        self
    }
    pub fn informative(mut self, informative: impl Into<String>) -> DropdownOption {
        self.informative = Some(informative.into());
        self
    }
    pub fn is_disabled(mut self) -> DropdownOption {
        self.is_disabled = true;
        self
    }
    pub fn is_disabled_if(mut self, is_disabled: bool) -> DropdownOption {
        self.is_disabled = is_disabled;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> DropdownOption {
        self.title = title.into();
        self
    }
}
/**TODO

**Example**

```rust #[derive(Debug, HasActionKey, Serialize, Deserialize)] pub enum Event { Foo }

ActionBlocker::new().body(Button::new("Disabled").event(Event::Foo)) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventBlocker {
    pub block: bool,
    pub body: Option<Component>,
}
#[allow(dead_code)]
impl EventBlocker {
    ///Construct a new EventBlocker.
    pub fn new(block: bool) -> EventBlocker {
        EventBlocker { block, body: None }
    }
    pub fn block(mut self) -> EventBlocker {
        self.block = true;
        self
    }
    pub fn block_if(mut self, block: bool) -> EventBlocker {
        self.block = block;
        self
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> EventBlocker {
        self.body = Some(body.into());
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flex {
    pub align: FlexAlign,
    pub gap: f64,
    pub items: Vec<FlexItem>,
    pub justify: FlexJustify,
    pub kind: FlexKind,
}
#[allow(dead_code)]
impl Flex {
    pub fn align(mut self, align: FlexAlign) -> Flex {
        self.align = align;
        self
    }
    pub fn gap(mut self, gap: f64) -> Flex {
        self.gap = gap;
        self
    }
    pub fn items(mut self, items: Vec<FlexItem>) -> Flex {
        self.items = items;
        self
    }
    pub fn justify(mut self, justify: FlexJustify) -> Flex {
        self.justify = justify;
        self
    }
    pub fn kind(mut self, kind: FlexKind) -> Flex {
        self.kind = kind;
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FlexAlign {
    Stretch,
    Center,
    Start,
    End,
    Baseline,
    SafeCenter,
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FlexGrowth {
    Auto,
    Expand,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexItem {
    pub component: Component,
    pub growth: FlexGrowth,
}
#[allow(dead_code)]
impl FlexItem {
    ///Construct a new FlexItem.
    pub fn new(
        component: impl objection::IntoComponentIndex<Index = Component>,
        growth: FlexGrowth,
    ) -> FlexItem {
        FlexItem {
            component: component.into(),
            growth,
        }
    }
    pub fn component(
        mut self,
        component: impl objection::IntoComponentIndex<Index = Component>,
    ) -> FlexItem {
        self.component = component.into();
        self
    }
    pub fn growth(mut self, growth: FlexGrowth) -> FlexItem {
        self.growth = growth;
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FlexJustify {
    Center,
    SafeCenter,
    Start,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FlexKind {
    Row,
    Column,
}
///A "nothing" component. Renders nothing.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fragment {}
#[allow(dead_code)]
impl Fragment {
    ///Construct a new Fragment.
    pub fn new() -> Fragment {
        Fragment {}
    }
}
/**A simple page layout, with a title, subtitle, some possible event items, and a body. Additionally, a logo can appear off to the right.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] pub enum Event { Foo, Bar, }

Flex::new(FlexKind::Column) .gap(30) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Large) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "A very long comment that will take up some notable space") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Medium) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .title_edit_event(Event::Foo) .subtitle_edit_event(Event::Bar) .subtitle_placeholder("No description") .size(HeaderSize::Small) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub event_items: Vec<HeaderActionItem>,
    pub size: HeaderSize,
    pub subtitle: Option<String>,
    pub subtitle_edit_event: Option<objection::EventKey<String>>,
    pub subtitle_placeholder: Option<String>,
    pub title: String,
    pub title_edit_event: Option<objection::EventKey<String>>,
    pub title_placeholder: Option<String>,
}
#[allow(dead_code)]
impl Header {
    ///Construct a new Header.
    pub fn new(
        event_items: Vec<HeaderActionItem>,
        size: HeaderSize,
        title: impl Into<String>,
    ) -> Header {
        Header {
            event_items,
            size,
            subtitle: None,
            subtitle_edit_event: None,
            subtitle_placeholder: None,
            title: title.into(),
            title_edit_event: None,
            title_placeholder: None,
        }
    }
    pub fn event_items(mut self, event_items: Vec<HeaderActionItem>) -> Header {
        self.event_items = event_items;
        self
    }
    pub fn size(mut self, size: HeaderSize) -> Header {
        self.size = size;
        self
    }
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Header {
        self.subtitle = Some(subtitle.into());
        self
    }
    pub fn subtitle_edit_event(
        mut self,
        subtitle_edit_event: objection::EventKey<String>,
    ) -> Header {
        self.subtitle_edit_event = Some(subtitle_edit_event);
        self
    }
    pub fn subtitle_placeholder(
        mut self,
        subtitle_placeholder: impl Into<String>,
    ) -> Header {
        self.subtitle_placeholder = Some(subtitle_placeholder.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Header {
        self.title = title.into();
        self
    }
    pub fn title_edit_event(
        mut self,
        title_edit_event: objection::EventKey<String>,
    ) -> Header {
        self.title_edit_event = Some(title_edit_event);
        self
    }
    pub fn title_placeholder(mut self, title_placeholder: impl Into<String>) -> Header {
        self.title_placeholder = Some(title_placeholder.into());
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderActionItem {
    pub event: objection::EventKey<()>,
    pub icon: IconName,
    pub label: String,
}
#[allow(dead_code)]
impl HeaderActionItem {
    ///Construct a new HeaderActionItem.
    pub fn new(
        event: objection::EventKey<()>,
        icon: IconName,
        label: impl Into<String>,
    ) -> HeaderActionItem {
        HeaderActionItem {
            event,
            icon,
            label: label.into(),
        }
    }
    pub fn event(mut self, event: objection::EventKey<()>) -> HeaderActionItem {
        self.event = event;
        self
    }
    pub fn icon(mut self, icon: IconName) -> HeaderActionItem {
        self.icon = icon;
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> HeaderActionItem {
        self.label = label.into();
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum HeaderSize {
    Large,
    Medium,
    Small,
}
/**TODO

**Example**

```rust Flex::new(FlexKind::Row) .gap(30) .justify(FlexJustify::Center) .align(FlexAlign::Center) .auto_item( Icon::new("mdi-ab-testing", 30).color(Color::Primary(100))) .auto_item( Icon::new("mdi-account-arrow-left", 30).color(Color::Success(100))) .auto_item( Icon::new("mdi-access-point", 30).color(Color::Danger(50))) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub color: Color,
    pub name: IconName,
    pub size: f64,
    pub title: Option<String>,
}
#[allow(dead_code)]
impl Icon {
    ///Construct a new Icon.
    pub fn new(color: Color, name: IconName, size: f64) -> Icon {
        Icon {
            color,
            name,
            size,
            title: None,
        }
    }
    pub fn color(mut self, color_opacity: f64, color_kind: ColorType) -> Icon {
        self.color = Color {
            opacity: color_opacity,
            kind: color_kind,
        };
        self
    }
    pub fn color_full(mut self, color: Color) -> Icon {
        self.color = color;
        self
    }
    pub fn name(mut self, name: IconName) -> Icon {
        self.name = name;
        self
    }
    pub fn size(mut self, size: f64) -> Icon {
        self.size = size;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Icon {
        self.title = Some(title.into());
        self
    }
}
/**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo }

Flex::new(FlexKind::Row) .gap(20) .auto_item( IconButton::new("mdi-ab-testing") .color(Color::Primary(100)) .title("A description of what this does and it is a rather long description") .size(40) .event(Event::Foo) ) .auto_item(IconButton::new("mdi-ab-testing")) .auto_item( IconButton::new("mdi-ab-testing") .color(Color::Primary(100)) .event(Event::Foo) ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconButton {
    pub event: Option<objection::EventKey<()>>,
    pub color: Color,
    pub name: IconName,
    pub size: f64,
    pub title: Option<String>,
}
#[allow(dead_code)]
impl IconButton {
    ///Construct a new IconButton.
    pub fn new(color: Color, name: IconName, size: f64) -> IconButton {
        IconButton {
            event: None,
            color,
            name,
            size,
            title: None,
        }
    }
    pub fn event(mut self, event: objection::EventKey<()>) -> IconButton {
        self.event = Some(event);
        self
    }
    pub fn color(mut self, color_opacity: f64, color_kind: ColorType) -> IconButton {
        self.color = Color {
            opacity: color_opacity,
            kind: color_kind,
        };
        self
    }
    pub fn color_full(mut self, color: Color) -> IconButton {
        self.color = color;
        self
    }
    pub fn name(mut self, name: IconName) -> IconButton {
        self.name = name;
        self
    }
    pub fn size(mut self, size: f64) -> IconButton {
        self.size = size;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> IconButton {
        self.title = Some(title.into());
        self
    }
}
///
pub type IconName = String;
/**TODO

**Example**

```rust Image::new("https://images.unsplash.com/photo-1711436470690-cf49602d1cf1?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D") .width(300) .height(300) .fit(ImageFit::Cover) .decorate() ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub decorate: bool,
    pub fit: ImageFit,
    pub height: Option<f64>,
    pub position: ImagePosition,
    pub url: String,
    pub width: Option<f64>,
}
#[allow(dead_code)]
impl Image {
    pub fn decorate(mut self) -> Image {
        self.decorate = true;
        self
    }
    pub fn decorate_if(mut self, decorate: bool) -> Image {
        self.decorate = decorate;
        self
    }
    pub fn fit(mut self, fit: ImageFit) -> Image {
        self.fit = fit;
        self
    }
    pub fn height(mut self, height: f64) -> Image {
        self.height = Some(height);
        self
    }
    pub fn position(mut self, position: ImagePosition) -> Image {
        self.position = position;
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Image {
        self.url = url.into();
        self
    }
    pub fn width(mut self, width: f64) -> Image {
        self.width = Some(width);
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ImageFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ImagePosition {
    Bottom,
    Center,
    Left,
    LeftBottom,
    LeftTop,
    Right,
    RightBottom,
    RightTop,
    Top,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputValidity {
    pub level: InputValidityLevel,
    pub message: Option<String>,
}
#[allow(dead_code)]
impl InputValidity {
    ///Construct a new InputValidity.
    pub fn new(level: InputValidityLevel) -> InputValidity {
        InputValidity {
            level,
            message: None,
        }
    }
    pub fn level(mut self, level: InputValidityLevel) -> InputValidity {
        self.level = level;
        self
    }
    pub fn message(mut self, message: impl Into<String>) -> InputValidity {
        self.message = Some(message.into());
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum InputValidityLevel {
    Valid,
    Invalid,
    Normal,
}
/**A simple label

**Example**

```rust #[derive(Serialize, Deserialize, HasActionKey)] enum Event { Foo }

Flex::new(FlexKind::Column) .gap(5) .justify(FlexJustify::Center) .align(FlexAlign::Center) .auto_item(Label::new("Some Label")) .auto_item(Label::new("Italic").italic()) .auto_item(Label::new("Bold").bold()) .auto_item(Label::new("Another Color").color(Color::Primary(100))) .auto_item(Label::new("This one is editable").edit_event(Event::Foo).color(Color::Primary(100))) .auto_item( Flex::new(FlexKind::Row) .auto_item(Label::new("And so is this").edit_event(Event::Foo)) .auto_item(Label::new("And this too (with a placeholder)").edit_event(Event::Foo).placeholder("This is the placeholder!!!! It is pretty long.")) ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub color: Option<Color>,
    pub edit_event: Option<objection::EventKey<String>>,
    pub is_bold: Option<bool>,
    pub is_italic: Option<bool>,
    pub placeholder: Option<String>,
    pub text: Option<String>,
}
#[allow(dead_code)]
impl Label {
    ///Construct a new Label.
    pub fn new() -> Label {
        Label {
            color: None,
            edit_event: None,
            is_bold: None,
            is_italic: None,
            placeholder: None,
            text: None,
        }
    }
    pub fn color(mut self, color_opacity: f64, color_kind: ColorType) -> Label {
        self.color = Some(Color {
            opacity: color_opacity,
            kind: color_kind,
        });
        self
    }
    pub fn color_full(mut self, color: Color) -> Label {
        self.color = Some(color);
        self
    }
    pub fn edit_event(mut self, edit_event: objection::EventKey<String>) -> Label {
        self.edit_event = Some(edit_event);
        self
    }
    pub fn is_bold(mut self) -> Label {
        self.is_bold = Some(true);
        self
    }
    pub fn is_bold_if(mut self, is_bold: bool) -> Label {
        self.is_bold = Some(is_bold);
        self
    }
    pub fn is_italic(mut self) -> Label {
        self.is_italic = Some(true);
        self
    }
    pub fn is_italic_if(mut self, is_italic: bool) -> Label {
        self.is_italic = Some(is_italic);
        self
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Label {
        self.placeholder = Some(placeholder.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Label {
        self.text = Some(text.into());
        self
    }
}
/**A modal that appears over all existing content, using the context from where it is placed.
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modal {
    pub body: Option<Component>,
    pub cancel_event: Option<objection::EventKey<()>>,
    pub cancel_event_label: Option<String>,
    pub description: Option<String>,
    pub finish_event: Option<objection::EventKey<()>>,
    pub finish_event_label: Option<String>,
    pub size: ModalSize,
    pub title: String,
}
#[allow(dead_code)]
impl Modal {
    ///Construct a new Modal.
    pub fn new(size: ModalSize, title: impl Into<String>) -> Modal {
        Modal {
            body: None,
            cancel_event: None,
            cancel_event_label: None,
            description: None,
            finish_event: None,
            finish_event_label: None,
            size,
            title: title.into(),
        }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> Modal {
        self.body = Some(body.into());
        self
    }
    pub fn cancel_event(mut self, cancel_event: objection::EventKey<()>) -> Modal {
        self.cancel_event = Some(cancel_event);
        self
    }
    pub fn cancel_event_label(mut self, cancel_event_label: impl Into<String>) -> Modal {
        self.cancel_event_label = Some(cancel_event_label.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Modal {
        self.description = Some(description.into());
        self
    }
    pub fn finish_event(mut self, finish_event: objection::EventKey<()>) -> Modal {
        self.finish_event = Some(finish_event);
        self
    }
    pub fn finish_event_label(mut self, finish_event_label: impl Into<String>) -> Modal {
        self.finish_event_label = Some(finish_event_label.into());
        self
    }
    pub fn size(mut self, size: ModalSize) -> Modal {
        self.size = size;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Modal {
        self.title = title.into();
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
}
/**TODO

**Indented Example**

```rust NestedFlow::new() .indent() .header("Created Alone") .content("Options", Label::new("Hi there!")) .content("Evil Plans", Label::new("Bad bad here")) .content("Good Plans", Label::new("Good good here!")) ```

**Not Indented Example**

```rust NestedFlow::new() .header("Created Alone") .content("Options", Label::new("Hi there!")) .content("Evil Plans", Label::new("Bad bad here")) .content("Good Plans", Label::new("Good good here!")) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedFlow {
    pub indent: bool,
    pub items: Vec<NestedFlowItem>,
}
#[allow(dead_code)]
impl NestedFlow {
    ///Construct a new NestedFlow.
    pub fn new(indent: bool, items: Vec<NestedFlowItem>) -> NestedFlow {
        NestedFlow { indent, items }
    }
    pub fn indent(mut self) -> NestedFlow {
        self.indent = true;
        self
    }
    pub fn indent_if(mut self, indent: bool) -> NestedFlow {
        self.indent = indent;
        self
    }
    pub fn items(mut self, items: Vec<NestedFlowItem>) -> NestedFlow {
        self.items = items;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedFlowContentItem {
    pub content: Component,
    pub header_text: String,
}
#[allow(dead_code)]
impl NestedFlowContentItem {
    ///Construct a new NestedFlowContentItem.
    pub fn new(
        content: impl objection::IntoComponentIndex<Index = Component>,
        header_text: impl Into<String>,
    ) -> NestedFlowContentItem {
        NestedFlowContentItem {
            content: content.into(),
            header_text: header_text.into(),
        }
    }
    pub fn content(
        mut self,
        content: impl objection::IntoComponentIndex<Index = Component>,
    ) -> NestedFlowContentItem {
        self.content = content.into();
        self
    }
    pub fn header_text(
        mut self,
        header_text: impl Into<String>,
    ) -> NestedFlowContentItem {
        self.header_text = header_text.into();
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NestedFlowHeaderItem {
    pub text: String,
}
#[allow(dead_code)]
impl NestedFlowHeaderItem {
    ///Construct a new NestedFlowHeaderItem.
    pub fn new(text: impl Into<String>) -> NestedFlowHeaderItem {
        NestedFlowHeaderItem {
            text: text.into(),
        }
    }
    pub fn text(mut self, text: impl Into<String>) -> NestedFlowHeaderItem {
        self.text = text.into();
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "def")]
pub enum NestedFlowItem {
    Header(NestedFlowHeaderItem),
    Content(NestedFlowContentItem),
}
#[allow(dead_code)]
impl NestedFlowItem {}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notice {
    pub message: String,
    pub style: NoticeStyle,
}
#[allow(dead_code)]
impl Notice {
    ///Construct a new Notice.
    pub fn new(message: impl Into<String>, style: NoticeStyle) -> Notice {
        Notice {
            message: message.into(),
            style,
        }
    }
    pub fn message(mut self, message: impl Into<String>) -> Notice {
        self.message = message.into();
        self
    }
    pub fn style(mut self, style: NoticeStyle) -> Notice {
        self.style = style;
        self
    }
}
/**A notice manager for displaying notices on the screen
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoticeManager {
    pub add_notice_action: objection::ActionKey<Notice>,
    pub body: Component,
}
#[allow(dead_code)]
impl NoticeManager {
    ///Construct a new NoticeManager.
    pub fn new(
        add_notice_action: objection::ActionKey<Notice>,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> NoticeManager {
        NoticeManager {
            add_notice_action,
            body: body.into(),
        }
    }
    pub fn add_notice_action(
        mut self,
        add_notice_action: objection::ActionKey<Notice>,
    ) -> NoticeManager {
        self.add_notice_action = add_notice_action;
        self
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> NoticeManager {
        self.body = body.into();
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum NoticeStyle {
    Error,
    Success,
}
/**A container with padding.

**Example**

```rust Padding::all(30).body(Card::new().body(Label::new("See, it is padded!"))) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Padding {
    pub body: Option<Component>,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub top: f64,
}
#[allow(dead_code)]
impl Padding {
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> Padding {
        self.body = Some(body.into());
        self
    }
    pub fn bottom(mut self, bottom: f64) -> Padding {
        self.bottom = bottom;
        self
    }
    pub fn left(mut self, left: f64) -> Padding {
        self.left = left;
        self
    }
    pub fn right(mut self, right: f64) -> Padding {
        self.right = right;
        self
    }
    pub fn top(mut self, top: f64) -> Padding {
        self.top = top;
        self
    }
}
/**A ui-decorated box for displaying content
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewBox {
    pub child: Component,
    pub title: String,
}
#[allow(dead_code)]
impl PreviewBox {
    ///Construct a new PreviewBox.
    pub fn new(
        child: impl objection::IntoComponentIndex<Index = Component>,
        title: impl Into<String>,
    ) -> PreviewBox {
        PreviewBox {
            child: child.into(),
            title: title.into(),
        }
    }
    pub fn child(
        mut self,
        child: impl objection::IntoComponentIndex<Index = Component>,
    ) -> PreviewBox {
        self.child = child.into();
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> PreviewBox {
        self.title = title.into();
        self
    }
}
/**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Batter, }

Flex::new(FlexKind::Column) .gap(30) .auto_item( RadioInput::new() .event(Event::Batter) .item(0, "Red") .item(1, "Green") ) .auto_item( RadioInput::new() .event(Event::Batter) .item(0, "Hi") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) .auto_item( RadioInput::new() .item(0, "all are disabled here") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioInput {
    pub event: Option<objection::EventKey<f64>>,
    pub initial_value: Option<f64>,
    pub items: Vec<RadioItem>,
}
#[allow(dead_code)]
impl RadioInput {
    ///Construct a new RadioInput.
    pub fn new(items: Vec<RadioItem>) -> RadioInput {
        RadioInput {
            event: None,
            initial_value: None,
            items,
        }
    }
    pub fn event(mut self, event: objection::EventKey<f64>) -> RadioInput {
        self.event = Some(event);
        self
    }
    pub fn initial_value(mut self, initial_value: f64) -> RadioInput {
        self.initial_value = Some(initial_value);
        self
    }
    pub fn items(mut self, items: Vec<RadioItem>) -> RadioInput {
        self.items = items;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioItem {
    pub description: Option<Component>,
    pub id: f64,
    pub title: String,
}
#[allow(dead_code)]
impl RadioItem {
    ///Construct a new RadioItem.
    pub fn new(id: f64, title: impl Into<String>) -> RadioItem {
        RadioItem {
            description: None,
            id,
            title: title.into(),
        }
    }
    pub fn description(
        mut self,
        description: impl objection::IntoComponentIndex<Index = Component>,
    ) -> RadioItem {
        self.description = Some(description.into());
        self
    }
    pub fn id(mut self, id: f64) -> RadioItem {
        self.id = id;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> RadioItem {
        self.title = title.into();
        self
    }
}
/**A scrollable box.

**Example**

```rust ScrollableBox::new().body(Padding::all(20).body("Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of \"de Finibus Bonorum et Malorum\" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, \"Lorem ipsum dolor sit amet..\", comes from a line in section 1.10.32. But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.")) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrollableBox {
    pub body: Option<Component>,
}
#[allow(dead_code)]
impl ScrollableBox {
    ///Construct a new ScrollableBox.
    pub fn new() -> ScrollableBox {
        ScrollableBox { body: None }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> ScrollableBox {
        self.body = Some(body.into());
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SelectionMode {
    OptIn,
    OptOut,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarGroup {
    pub items: Vec<SidebarItem>,
    pub name: String,
}
#[allow(dead_code)]
impl SidebarGroup {
    ///Construct a new SidebarGroup.
    pub fn new(items: Vec<SidebarItem>, name: impl Into<String>) -> SidebarGroup {
        SidebarGroup {
            items,
            name: name.into(),
        }
    }
    pub fn items(mut self, items: Vec<SidebarItem>) -> SidebarGroup {
        self.items = items;
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> SidebarGroup {
        self.name = name.into();
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarItem {
    pub event: Option<objection::EventKey<()>>,
    pub icon: Option<IconName>,
    pub title: String,
}
#[allow(dead_code)]
impl SidebarItem {
    ///Construct a new SidebarItem.
    pub fn new(title: impl Into<String>) -> SidebarItem {
        SidebarItem {
            event: None,
            icon: None,
            title: title.into(),
        }
    }
    pub fn event(mut self, event: objection::EventKey<()>) -> SidebarItem {
        self.event = Some(event);
        self
    }
    pub fn icon(mut self, icon: IconName) -> SidebarItem {
        self.icon = Some(icon);
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> SidebarItem {
        self.title = title.into();
        self
    }
}
/**A sidebar application layout.

**Example**

```rust SidebarLayout::new("Abc Corp") ```

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Action { Foo, Bar, }

SidebarLayout::new("Abc Corp") .title_event(Action::Foo) .logo(Image::new("https://github.githubassets.com/assets/3m-0151c2fda0ce.svg").width(30).height(30)) .event_item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .event_item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) .group( SidebarGroup::new("Main") .item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) ) .group( SidebarGroup::new("Records") .item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) ) .initial_event(Action::Foo) .footer(Center::new().body("Za feetsies")) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarLayout {
    pub event_items: Vec<SidebarItem>,
    pub body: Option<Component>,
    pub footer: Option<Component>,
    pub groups: Vec<SidebarGroup>,
    pub initial_event: Option<objection::EventKey<()>>,
    pub logo: Option<Image>,
    pub title: String,
    pub title_event: Option<objection::EventKey<()>>,
}
#[allow(dead_code)]
impl SidebarLayout {
    ///Construct a new SidebarLayout.
    pub fn new(
        event_items: Vec<SidebarItem>,
        groups: Vec<SidebarGroup>,
        title: impl Into<String>,
    ) -> SidebarLayout {
        SidebarLayout {
            event_items,
            body: None,
            footer: None,
            groups,
            initial_event: None,
            logo: None,
            title: title.into(),
            title_event: None,
        }
    }
    pub fn event_items(mut self, event_items: Vec<SidebarItem>) -> SidebarLayout {
        self.event_items = event_items;
        self
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> SidebarLayout {
        self.body = Some(body.into());
        self
    }
    pub fn footer(
        mut self,
        footer: impl objection::IntoComponentIndex<Index = Component>,
    ) -> SidebarLayout {
        self.footer = Some(footer.into());
        self
    }
    pub fn groups(mut self, groups: Vec<SidebarGroup>) -> SidebarLayout {
        self.groups = groups;
        self
    }
    pub fn initial_event(
        mut self,
        initial_event: objection::EventKey<()>,
    ) -> SidebarLayout {
        self.initial_event = Some(initial_event);
        self
    }
    pub fn logo(mut self, logo: Image) -> SidebarLayout {
        self.logo = Some(logo);
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> SidebarLayout {
        self.title = title.into();
        self
    }
    pub fn title_event(mut self, title_event: objection::EventKey<()>) -> SidebarLayout {
        self.title_event = Some(title_event);
        self
    }
}
/**TODO

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo }

Padding::all(10) .body( Flex::new(FlexKind::Column) .gap(10) .auto_item( Skeleton::new( Event::Foo, Card::new().body( Flex::new(FlexKind::Column) .gap(10) .auto_item( RadioInput::new() .item(0, "Hi") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) .auto_item( Image::new("https://images.unsplash.com/photo-1716369415085-4a6876f91840?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxlZGl0b3JpYWwtZmVlZHwyfHx8ZW58MHx8fHx8") .width(300) .height(200) .fit(ImageFit::Cover) .decorate() ) ) ) ) .auto_item(Button::new("Load").event(Event::Foo)) ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skeleton {
    pub body: Component,
    pub linked_event: AnyEvent,
}
#[allow(dead_code)]
impl Skeleton {
    ///Construct a new Skeleton.
    pub fn new(
        body: impl objection::IntoComponentIndex<Index = Component>,
        linked_event: AnyEvent,
    ) -> Skeleton {
        Skeleton {
            body: body.into(),
            linked_event,
        }
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> Skeleton {
        self.body = body.into();
        self
    }
    pub fn linked_event(mut self, linked_event_action_path: Vec<String>) -> Skeleton {
        self.linked_event = AnyEvent {
            action_path: linked_event_action_path,
            debug_symbol: None,
        };
        self
    }
    pub fn linked_event_full(mut self, linked_event: AnyEvent) -> Skeleton {
        self.linked_event = linked_event;
        self
    }
}
/**TODO

**Example**

```rust Table::new() .column("Id") .expanded_column("Name") .column("") .rows(Vec::from([ Vec::<Component>::from([ Label::new("82").into(), Label::new("Jason").into(), Button::new("View").size(ButtonSize::Small).into() ]), Vec::<Component>::from([ Label::new("84").into(), Label::new("James").into(), Button::new("View").size(ButtonSize::Small).into() ]), Vec::<Component>::from([ Label::new("103").into(), Label::new("Jeehoshofat Bartholemew, Duke of Northumberland, King of \"The Rose Garden\", the sixteenth").into(), Button::new("View").size(ButtonSize::Small).into() ]), ])) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub columns: Vec<TableColumn>,
    pub rows: Vec<Vec<Component>>,
}
#[allow(dead_code)]
impl Table {
    ///Construct a new Table.
    pub fn new(columns: Vec<TableColumn>, rows: Vec<Vec<Component>>) -> Table {
        Table { columns, rows }
    }
    pub fn columns(mut self, columns: Vec<TableColumn>) -> Table {
        self.columns = columns;
        self
    }
    pub fn rows(mut self, rows: Vec<Vec<Component>>) -> Table {
        self.rows = rows;
        self
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumn {
    pub expand: bool,
    pub name: String,
}
#[allow(dead_code)]
impl TableColumn {
    ///Construct a new TableColumn.
    pub fn new(expand: bool, name: impl Into<String>) -> TableColumn {
        TableColumn {
            expand,
            name: name.into(),
        }
    }
    pub fn expand(mut self) -> TableColumn {
        self.expand = true;
        self
    }
    pub fn expand_if(mut self, expand: bool) -> TableColumn {
        self.expand = expand;
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> TableColumn {
        self.name = name.into();
        self
    }
}
/**A text input. If no change event, blur event, or dropdown selection event is supplied, the input will be disabled.

If some initial dropdown options are supplied, but no `change_event` is supplied, the dropdown options will be sorted locally. If a `change_event` is supplied, the server is expected to send down a new list of dropdown options.

If no `option_selection_event` is supplied, the selected dropdown options will simply replace the input value, triggering the default value update behavior.

`allow_multiple_options` has no effect if an `option_selected_option` is not supplied. If it is, more that one option can be selected.

**Example**

```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { InputChanged, InputBlurred, OptionSelected, Submit }

Padding::all(30).body( Flex::new(FlexKind::Column) .gap(20) .auto_item(TextInput::new("Username").change_event(Event::InputChanged).submit_event(Event::Submit)) .auto_item(TextInput::new("Password").role(TextInputRole::Password).blur_event(Event::InputBlurred).submit_event(Event::Submit)) .auto_item(TextInput::new("With Initial Value").initial_value("Hello there!").blur_event(Event::InputBlurred).submit_event(Event::Submit)) .auto_item(TextInput::new("Email (disabled)").submit_event(Event::Submit).role(TextInputRole::Email).leading_icon("mdi-ab-testing")) .auto_item( TextInput::new("Dropdown with client filtering") .role(TextInputRole::Email) .blur_event(Event::InputBlurred) .submit_event(Event::Submit) .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3"), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) .auto_item( TextInput::new("Dropdown with server filtering") .role(TextInputRole::Email) .change_event(Event::InputChanged) .submit_event(Event::Submit) .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3").is_disabled(), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) .auto_item( TextInput::new("Dropdown without free text and client filtering") .role(TextInputRole::Email) .option_selected_event(Event::OptionSelected) .submit_event(Event::Submit) .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3"), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) .auto_item( TextInput::new("Dropdown without free text and client filtering and multiple") .role(TextInputRole::Email) .option_selected_event(Event::OptionSelected) .submit_event(Event::Submit) .multiple() .initial_dropdown_options(Vec::from([ DropdownOption::new(Uuid::new_v4(), "Option 1"), DropdownOption::new(Uuid::new_v4(), "Option 2"), DropdownOption::new(Uuid::new_v4(), "Option 3"), DropdownOption::new(Uuid::new_v4(), "Option 4"), DropdownOption::new(Uuid::new_v4(), "Option 5"), ])) ) ) ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextInput {
    pub blur_event: Option<objection::EventKey<String>>,
    pub change_event: Option<objection::EventKey<String>>,
    pub initial_dropdown_options: Option<Vec<DropdownOption>>,
    pub initial_selected_options: Option<Vec<String>>,
    pub initial_value: Option<String>,
    pub label: String,
    pub leading_icon: Option<IconName>,
    pub multiple: bool,
    pub option_selected_event: Option<objection::EventKey<Vec<String>>>,
    pub role: TextInputRole,
    pub submit_event: Option<objection::EventKey<()>>,
    pub trailing_icon: Option<IconName>,
    pub default_validity: Option<InputValidity>,
    pub set_options_action: Option<objection::ActionKey<Vec<DropdownOption>>>,
    pub set_validity_action: Option<objection::ActionKey<InputValidity>>,
}
#[allow(dead_code)]
impl TextInput {
    ///Construct a new TextInput.
    pub fn new(
        label: impl Into<String>,
        multiple: bool,
        role: TextInputRole,
    ) -> TextInput {
        TextInput {
            blur_event: None,
            change_event: None,
            initial_dropdown_options: None,
            initial_selected_options: None,
            initial_value: None,
            label: label.into(),
            leading_icon: None,
            multiple,
            option_selected_event: None,
            role,
            submit_event: None,
            trailing_icon: None,
            default_validity: None,
            set_options_action: None,
            set_validity_action: None,
        }
    }
    pub fn blur_event(mut self, blur_event: objection::EventKey<String>) -> TextInput {
        self.blur_event = Some(blur_event);
        self
    }
    pub fn change_event(
        mut self,
        change_event: objection::EventKey<String>,
    ) -> TextInput {
        self.change_event = Some(change_event);
        self
    }
    pub fn initial_dropdown_options(
        mut self,
        initial_dropdown_options: Vec<DropdownOption>,
    ) -> TextInput {
        self.initial_dropdown_options = Some(initial_dropdown_options);
        self
    }
    pub fn initial_selected_options(
        mut self,
        initial_selected_options: Vec<String>,
    ) -> TextInput {
        self.initial_selected_options = Some(initial_selected_options);
        self
    }
    pub fn initial_value(mut self, initial_value: impl Into<String>) -> TextInput {
        self.initial_value = Some(initial_value.into());
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> TextInput {
        self.label = label.into();
        self
    }
    pub fn leading_icon(mut self, leading_icon: IconName) -> TextInput {
        self.leading_icon = Some(leading_icon);
        self
    }
    pub fn multiple(mut self) -> TextInput {
        self.multiple = true;
        self
    }
    pub fn multiple_if(mut self, multiple: bool) -> TextInput {
        self.multiple = multiple;
        self
    }
    pub fn option_selected_event(
        mut self,
        option_selected_event: objection::EventKey<Vec<String>>,
    ) -> TextInput {
        self.option_selected_event = Some(option_selected_event);
        self
    }
    pub fn role(mut self, role: TextInputRole) -> TextInput {
        self.role = role;
        self
    }
    pub fn submit_event(mut self, submit_event: objection::EventKey<()>) -> TextInput {
        self.submit_event = Some(submit_event);
        self
    }
    pub fn trailing_icon(mut self, trailing_icon: IconName) -> TextInput {
        self.trailing_icon = Some(trailing_icon);
        self
    }
    pub fn default_validity(
        mut self,
        default_validity_level: InputValidityLevel,
    ) -> TextInput {
        self.default_validity = Some(InputValidity {
            level: default_validity_level,
            message: None,
        });
        self
    }
    pub fn default_validity_full(
        mut self,
        default_validity: InputValidity,
    ) -> TextInput {
        self.default_validity = Some(default_validity);
        self
    }
    pub fn set_options_action(
        mut self,
        set_options_action: objection::ActionKey<Vec<DropdownOption>>,
    ) -> TextInput {
        self.set_options_action = Some(set_options_action);
        self
    }
    pub fn set_validity_action(
        mut self,
        set_validity_action: objection::ActionKey<InputValidity>,
    ) -> TextInput {
        self.set_validity_action = Some(set_validity_action);
        self
    }
}
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TextInputRole {
    Plain,
    Password,
    Email,
    Search,
    Url,
    Tel,
    Numeric,
    Decimal,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub dark_palette: ColorPalette,
    pub default_font: Option<String>,
    pub fancy_font: Option<String>,
    pub light_palette: ColorPalette,
    pub round_base: bool,
    pub selection_mode: SelectionMode,
    pub window_scrolling: bool,
}
#[allow(dead_code)]
impl Theme {
    pub fn dark_palette(mut self, dark_palette: ColorPalette) -> Theme {
        self.dark_palette = dark_palette;
        self
    }
    pub fn default_font(mut self, default_font: impl Into<String>) -> Theme {
        self.default_font = Some(default_font.into());
        self
    }
    pub fn fancy_font(mut self, fancy_font: impl Into<String>) -> Theme {
        self.fancy_font = Some(fancy_font.into());
        self
    }
    pub fn light_palette(mut self, light_palette: ColorPalette) -> Theme {
        self.light_palette = light_palette;
        self
    }
    pub fn round_base(mut self) -> Theme {
        self.round_base = true;
        self
    }
    pub fn round_base_if(mut self, round_base: bool) -> Theme {
        self.round_base = round_base;
        self
    }
    pub fn selection_mode(mut self, selection_mode: SelectionMode) -> Theme {
        self.selection_mode = selection_mode;
        self
    }
    pub fn window_scrolling(mut self) -> Theme {
        self.window_scrolling = true;
        self
    }
    pub fn window_scrolling_if(mut self, window_scrolling: bool) -> Theme {
        self.window_scrolling = window_scrolling;
        self
    }
}
/**A theme manager for all components. Currently the theme of all components, regardless of location are affected, but this is expected to change
to where only child components are affected.
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeManager {
    pub theme: Theme,
    pub body: Component,
}
#[allow(dead_code)]
impl ThemeManager {
    ///Construct a new ThemeManager.
    pub fn new(
        theme: Theme,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> ThemeManager {
        ThemeManager {
            theme,
            body: body.into(),
        }
    }
    pub fn theme(mut self, theme: Theme) -> ThemeManager {
        self.theme = theme;
        self
    }
    pub fn body(
        mut self,
        body: impl objection::IntoComponentIndex<Index = Component>,
    ) -> ThemeManager {
        self.body = body.into();
        self
    }
}
/**A component for setting the a title segment. Will be joined to any title segments in the component hierarchy using a '|'
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleSegment {
    pub title: String,
}
#[allow(dead_code)]
impl TitleSegment {
    ///Construct a new TitleSegment.
    pub fn new(title: impl Into<String>) -> TitleSegment {
        TitleSegment {
            title: title.into(),
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> TitleSegment {
        self.title = title.into();
        self
    }
}
/**A boundary that allows it's children to be updated without beaming down an entirely new widget tree.

You can optionally set a child to be displayed until an update is sent. Once an update has been sent has been sent, the original child will never be rendered. ```
*/
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBoundary {
    pub child: Option<Component>,
    pub action: objection::ActionKey<Component>,
}
#[allow(dead_code)]
impl UpdateBoundary {
    ///Construct a new UpdateBoundary.
    pub fn new(action: objection::ActionKey<Component>) -> UpdateBoundary {
        UpdateBoundary {
            child: None,
            action,
        }
    }
    pub fn child(
        mut self,
        child: impl objection::IntoComponentIndex<Index = Component>,
    ) -> UpdateBoundary {
        self.child = Some(child.into());
        self
    }
    pub fn action(mut self, action: objection::ActionKey<Component>) -> UpdateBoundary {
        self.action = action;
        self
    }
}
