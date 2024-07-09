import { CircleProgress, React } from 'runtime'
import { LabelRender } from '../label/mod.tsx'

export function CircleProgressRender(props: CircleProgress) {
	const circleSize = props.size - 10
	const circumference = Math.PI * circleSize
	const [fillAmount, setFillAmount] = React.useState(0)

	React.useEffect(() => {
		setFillAmount(props.value)
	}, [])

	return (
		<div class={`relative w-${props.size} h-${props.size}`}>
			<svg
				width={props.size}
				height={props.size}
				xmlns='http://www.w3.org/2000/svg'
				style='transform:rotate(-90deg)'
			>
				<circle
					class='stroke-fore-10'
					r={circleSize / 2}
					cx={props.size / 2}
					cy={props.size / 2}
					fill='transparent'
					stroke-width='6'
					stroke-dashoffset='0'
				>
				</circle>
				<circle
					class='stroke-primary transition-all delay-1000 duration-1000 ease-in-out'
					r={circleSize / 2}
					cx={props.size / 2}
					cy={props.size / 2}
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
