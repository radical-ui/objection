import { React } from './deps.ts'
import { Label, LabelRender } from './label.tsx'

/**
 * TODO
 *
 * **Example**
 *
 * ```rust CircleProgress::new() .value(0.5) .label("Hello") ```
 *
 * @component
 */
export interface CircleProgress {
	label: Label
	value: number

	size?: number
}

export function CircleProgressRender(props: CircleProgress) {
	const size = props.size ?? 100
	const circleSize = size - 10
	const circumference = Math.PI * circleSize
	const [fillAmount, setFillAmount] = React.useState(0)

	React.useEffect(() => {
		setFillAmount(props.value)
	}, [])

	return (
		<div class={`relative w-${size} h-${size}`}>
			<svg
				width={size}
				height={size}
				xmlns='http://www.w3.org/2000/svg'
				style='transform:rotate(-90deg)'
			>
				<circle
					class='stroke-fore-10'
					r={circleSize / 2}
					cx={size / 2}
					cy={size / 2}
					fill='transparent'
					stroke-width='6'
					stroke-dashoffset='0'
				>
				</circle>
				<circle
					class='stroke-primary transition-all delay-1000 duration-1000 ease-in-out'
					r={circleSize / 2}
					cx={size / 2}
					cy={size / 2}
					stroke-width='8'
					stroke-linecap='round'
					stroke-dashoffset={`${circumference - fillAmount * circumference}px`}
					fill='transparent'
					stroke-dasharray={`${circumference}px`}
				>
				</circle>
			</svg>
			<div class='absolute inset-0 flex items-center justify-center font-semibold text-xl'>
				<LabelRender {...props.label} />
			</div>
		</div>
	)
}
