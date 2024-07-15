import { React } from './deps.ts'
import { IconButtonRender, IconName } from './icon.tsx'
import { LabelRender } from './label.tsx'
import { ActionKey } from './types.ts'

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
	action: ActionKey
	icon: IconName
	label: string
}

/**
 * A simple page layout, with a title, subtitle, some possible action items, and a body. Additionally, a logo can appear off to the right.
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] pub enum Event { Foo, Bar, }
 *
 * Flex::new(FlexKind::Column) .gap(30) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Large) .action_item(Event::Foo, "mdi-pencil", "Do Foo") .action_item(Event::Bar, "mdi-ab-testing", "A very long comment that will take up some notable space") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .size(HeaderSize::Medium) .action_item(Event::Foo, "mdi-pencil", "Do Foo") .action_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) .auto_item( Header::new("With Action Items") .subtitle("A subtitle here") .title_edit_action(Event::Foo) .subtitle_edit_action(Event::Bar) .subtitle_placeholder("No description") .size(HeaderSize::Small) .action_item(Event::Foo, "mdi-pencil", "Do Foo") .action_item(Event::Bar, "mdi-ab-testing", "Do Bar") ) ```
 *
 * @component
 */
export interface Header {
	action_items: HeaderActionItem[]
	size: HeaderSize
	subtitle?: string
	subtitle_edit_action?: ActionKey
	subtitle_placeholder?: string
	title: string
	title_edit_action?: ActionKey
	title_placeholder?: string
}

export function HeaderRender(props: Header) {
	return (
		<div class='flex-1 flex flex-col gap-5'>
			<div class='flex gap-5 items-center'>
				<h1 class={`flex-1 text-primary flex text-${getTextSize(props.size)}`}>
					<LabelRender
						color={{ type: 'Fore', opacity: 90 }}
						is_bold
						is_italic={false}
						text={props.title}
						edit_action={props.title_edit_action}
						placeholder={props.title_placeholder}
					/>
				</h1>
				{props.action_items.map((item) => (
					<div class='h-0 flex items-center'>
						<IconButtonRender
							color={{ type: 'Primary', opacity: 100 }}
							name={item.icon}
							size={getIconSize(props.size)}
							action={item.action}
							title={item.label}
						/>
					</div>
				))}
			</div>
			{props.subtitle && (
				<h3 class='text-fore-30 flex'>
					<LabelRender
						text={props.subtitle}
						is_bold={false}
						is_italic={false}
						color={{ type: 'Fore', opacity: 30 }}
						edit_action={props.subtitle_edit_action}
						placeholder={props.subtitle_placeholder}
					/>
				</h3>
			)}
		</div>
	)
}
