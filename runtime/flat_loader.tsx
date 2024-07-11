import { React } from './deps.ts'
import { Color } from './types.ts'
import { getColor, GlobalCss } from './utils.ts'

const css = new GlobalCss(`
	@keyframes increase {
		from { left: -5%; width: 5%; }
		to { left: 130%; width: 100%;}
	}
	@keyframes decrease {
		from { left: -80%; width: 80%; }
		to { left: 110%; width: 10%;}
	}
`)

export interface FlatLoaderProps {
	color: Color
	size: number
}

export function FlatLoader(props: FlatLoaderProps) {
	css.present()

	return (
		<div class={`relative h-${Math.round(props.size)}`}>
			<div class={`absolute inset-0 bg-${getColor(props.color, 30)}`} />
			<div class={`absolute top-0 bottom-0 bg-${getColor(props.color)}`} style={{ animation: 'increase 2s infinite' }} />
			<div class={`absolute top-0 bottom-0 bg-${getColor(props.color)}`} style={{ animation: 'decrease 2s 0.5s infinite' }} />
		</div>
	)
}
