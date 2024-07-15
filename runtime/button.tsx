import { useDispatcher } from './action.tsx'
import { doBubble } from './bubble.ts'
import { React } from './deps.ts'
import { IconName, IconRender } from './icon.tsx'
import { Spinner } from './spinner.tsx'
import { Color } from './types.ts'
import { ActionKey } from './types.ts'
import { getColor } from './utils.ts'

export type ButtonSize = 'Small' | 'Medium' | 'Large'

/**
 * A button that has a label and an action.
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, }
 *
 * Flex::new(FlexKind::Column) .gap(10) .align(FlexAlign::Center) .justify(FlexJustify::Center) .auto_item( Flex::new(FlexKind::Row) .gap(10) .align(FlexAlign::Center) .auto_item( Button::new("Small Button") .action(Event::Foo) .size(ButtonSize::Small) ) .auto_item( Button::new("Medium Button") .action(Event::Foo) ) .auto_item( Button::new("Large Button") .action(Event::Bar) .size(ButtonSize::Large) ) ) .auto_item( Flex::new(FlexKind::Row) .gap(10) .auto_item( Button::new("Fore Button") .action(Event::Foo) .color(Color::Fore(5)) ) .auto_item( Button::new("Success Button") .action(Event::Foo) .color(Color::Success(100)) ) .auto_item( Button::new("Danger Button") .action(Event::Foo) .color(Color::Danger(100)) ) ) .auto_item( Flex::new(FlexKind::Row) .gap(10) .auto_item( Button::new("Leading Icon") .action(Event::Foo) .leading_icon("mdi-ab-testing") ) .auto_item( Button::new("Trailing Icon") .action(Event::Foo) .trailing_icon("mdi-ab-testing") ) .auto_item( Button::new("Both") .action(Event::Bar) .trailing_icon("mdi-ab-testing") .leading_icon("mdi-ab-testing") .outline() ) ) ```
 *
 * @component
 */
export interface Button {
	action?: ActionKey
	color: Color
	full: boolean
	label: string
	leading_icon?: IconName
	outline: boolean
	size: ButtonSize
	trailing_icon?: IconName
}

export function ButtonRender(props: Button) {
	const { isLoading, dispatch, isDisabled: isActionDisabled } = useDispatcher(props.action ?? null)

	const scale = props.size === 'Large' ? 1 : props.size === 'Small' ? 0.6 : 0.8
	const isDisabled = isActionDisabled || isLoading

	const innerColor: Color = props.outline
		? props.color
		: props.color.type === 'Base' || props.color.type === 'Fore'
		? { type: 'Fore', opacity: 100 }
		: { type: 'DecorationFore', opacity: 100 }

	const textColor = `text-${getColor(innerColor, 100)}`

	const backgroundStyles = props.outline
		? `bg-transparent ${!isDisabled ? 'hover:bg-fore-10' : ''}`
		: `bg-${getColor(props.color)} ${!isDisabled ? `hover:bg-${getColor(props.color, -10)}` : ''} transition-colors`
	const borderStyles = props.outline ? `border border-${getColor(props.color)}` : ''

	return (
		<button
			class={`
				px-${Math.round(scale * 14)} py-${Math.round(scale * 8)} rounded
				${backgroundStyles} ${borderStyles} transition-colors
				uppercase font-semibold relative overflow-hidden
				focus:ring-4 ring-${getColor(props.color, 40)}
				${textColor}
				${props.full ? 'w-full' : ''}
				${isDisabled ? 'opacity-50 cursor-not-allowed' : ''}
			`}
			type='button'
			disabled={isDisabled}
			onClick={(event) => {
				if (!props.action) return console.error('button was clicked while disabled')

				doBubble(event.currentTarget, event)
				dispatch(null)
			}}
		>
			<div
				class={`
					flex gap-${Math.round(scale * 8)} items-center ${isLoading ? `opacity-0` : ''}
					${props.full ? 'w-full justify-center' : ''}`}
			>
				{props.leading_icon && <IconRender name={props.leading_icon} size={Math.round(scale * 18)} color={innerColor} />}
				<div class={`${props.size === 'Large' ? 'text-lg' : ''} ${props.size === 'Small' ? 'text-sm' : ''}`}>{props.label}</div>
				{props.trailing_icon && <IconRender name={props.trailing_icon} size={Math.round(scale * 18)} color={innerColor} />}
			</div>

			{isLoading
				? (
					<div class='inset-0 absolute flex justify-center items-center '>
						<Spinner color={innerColor} size={props.size === 'Large' ? 30 : props.size === 'Medium' ? 25 : 20} />
					</div>
				)
				: <></>}
		</button>
	)
}
