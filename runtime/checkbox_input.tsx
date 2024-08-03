import { useDispatcher } from './event.tsx'
import { EventKey, React } from './deps.ts'
import { IconRender } from './icon.tsx'

/**
 * A checkbox input, which can be either on or off.
 *
 * At some point, this component should be combined with a sort of shared context on the frontend to connect with other checkboxes, define roots, and be in an intermediate state.
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Action { Foo }
 *
 * Flex::new(FlexKind::Column) .auto_item(CheckboxInput::new("Allow tracking").initial_value(true).event(Action::Foo)) .auto_item(CheckboxInput::new("Allow tracking (disabled)").initial_value(false)) ```
 *
 * @component
 */
export interface CheckboxInput {
	event?: EventKey<boolean>
	checked?: boolean
	label: string
}

export function CheckboxInputRender(props: CheckboxInput) {
	const { dispatch, isDisabled } = useDispatcher(props.event ?? null)
	const [checked, setChecked] = React.useState(props.checked ?? false)

	return (
		<label class={`flex gap-5 items-center ${isDisabled ? 'def-50 cursor-not-allowed' : 'cursor-pointer'}`}>
			<input
				type='checkbox'
				checked={checked}
				disabled={isDisabled}
				class='hidden'
				onChange={() => {
					if (!props.event) return

					dispatch(!checked)
					setChecked(!checked)
				}}
			/>

			<JustTheCheckbox checked={checked} />

			<div class='select-none'>{props.label}</div>
		</label>
	)
}

export interface JustTheCheckboxProps {
	checked: boolean
}

export function JustTheCheckbox(props: JustTheCheckboxProps) {
	return (
		<div
			class={`
				rounded w-20 h-20 flex items-center justify-center border transition-colors
				${props.checked ? 'bg-primary border-primary' : 'bg-transparent border-fore-10'}
			`}
		>
			{props.checked && <IconRender color={{ type: 'DecorationFore', def: 100 }} name='mdi-check-bold' size={24} />}
		</div>
	)
}
