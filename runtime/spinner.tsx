import { React } from './deps.ts'
import { Color } from './theme.tsx'
import { getColor } from './utils.ts'

export interface SpinnerProps {
	color: Color
	size: number
}

export function Spinner(props: SpinnerProps) {
	return (
		<div
			class={`
				w-${Math.round(props.size)} h-${Math.round(props.size)}
				border-md border-${getColor(props.color, 10)} border-t-${getColor(props.color)} animate-spin
			`}
			style={{ borderRadius: '50%' }}
		/>
	)
}
