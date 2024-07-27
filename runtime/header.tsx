import { EventKey, React } from './deps.ts'
import { IconButtonRender, IconName } from './icon.tsx'
import { LabelRender } from './label.tsx'

const getIconSize = (size: HeaderSize) => {
	if (size === 'Large') return 25
	if (size === 'Medium') return 20
	if (size === 'Small') return 20

	throw new Error('unknown size')
}

const getTextSize = (size: HeaderSize) => {
	if (size === 'Large') return '2xl'
	if (size === 'Medium') return 'xl'
	if (size === 'Small') return 'lg'
}

export type HeaderSize = 'Large' | 'Medium' | 'Small'

export interface HeaderActionItem {
	event: EventKey<null>
	icon: IconName
	label: string
}

/**
 * A simple page layout, with a title, subtitle, some possible event items, and a body. Additionally, a logo can appear off to the right.
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] pub enum Event { Foo, Bar, }
 *
 * Flex::new(FlexKind::Column) .gap(30) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Large) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "A very long comment that will take up some notable space") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Medium) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .title_edit_event(Event::Foo) .subtitle_edit_event(Event::Bar) .subtitle_placeholder("No description") .size(HeaderSize::Small) .event_item(Event::Foo, "mdi-pencil", "Do Foo") .event_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) ```
 *
 * @component
 */
export interface Header {
	eventItems: HeaderActionItem[]
	size: HeaderSize
	subtitle?: string
	subtitleEditEvent?: EventKey<string>
	subtitlePlaceholder?: string
	title: string
	titleEditEvent?: EventKey<string>
	titlePlaceholder?: string
}

export function HeaderRender(props: Header) {
	return (
		<div class='flex-1 flex flex-col gap-5'>
			<div class='flex gap-5 items-center'>
				<h1 class={`flex-1 text-primary flex text-${getTextSize(props.size)}`}>
					<LabelRender
						color={{ kind: 'Fore', opacity: 90 }}
						isBold
						isItalic={false}
						text={props.title}
						editEvent={props.titleEditEvent}
						placeholder={props.titlePlaceholder}
					/>
				</h1>
				{props.eventItems.map((item) => (
					<div class='h-0 flex items-center'>
						<IconButtonRender
							color={{ kind: 'Primary', opacity: 100 }}
							name={item.icon}
							size={getIconSize(props.size)}
							event={item.event}
							title={item.label}
						/>
					</div>
				))}
			</div>
			{props.subtitle && (
				<h3 class='text-fore-30 flex'>
					<LabelRender
						text={props.subtitle}
						isBold={false}
						isItalic={false}
						color={{ kind: 'Fore', opacity: 30 }}
						editEvent={props.subtitleEditEvent}
						placeholder={props.subtitlePlaceholder}
					/>
				</h3>
			)}
		</div>
	)
}
