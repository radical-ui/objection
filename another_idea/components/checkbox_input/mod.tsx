import { CheckboxInput, isOk, React, useDispatcher } from 'runtime'
import { IconRender } from '../icon/mod.tsx'

export function CheckboxInputRender(props: CheckboxInput) {
	const { dispatch, isDisabled } = useDispatcher(props.action ?? null)
	const [checked, setChecked] = React.useState(props.initial_value)

	return (
		<label class={`flex gap-5 items-center ${isDisabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}>
			<input
				type='checkbox'
				checked={checked}
				disabled={isDisabled}
				class='hidden'
				onChange={() => {
					if (!isOk(props.action)) return

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
			{props.checked && <IconRender color={{ type: 'DecorationFore', opacity: 100 }} name='mdi-check-bold' size={24} />}
		</div>
	)
}
