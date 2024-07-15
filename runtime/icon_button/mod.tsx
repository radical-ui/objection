import { doBubble, getColor, IconButton, React, Spinner, Tooltip, useDispatcher } from 'runtime'
import { IconRender } from '../icon/mod.tsx'

export function IconButtonRender(props: IconButton & { handleClick?(): void }) {
	const { isLoading, dispatch, isDisabled: isActionDisabled } = useDispatcher(props.action ?? null)

	const isDisabled = isActionDisabled || isLoading
	const size = Math.round(props.size * 1.5)

	const backgroundStyle = `bg-fore-0 hover:bg-${getColor(props.color, 5)} focus:bg-${getColor(props.color, 10)}`

	return (
		// set the tooltip right at the edge of the icon
		<Tooltip text={props.title ?? null} offset={props.size * -0.25}>
			<button
				disabled={isDisabled}
				class={`
					transition-opacity ${isDisabled ? 'cursor-not-allowed opacity-50' : backgroundStyle}
					w-${size} h-${size} flex items-center justify-center relative
					focus:ring-4 ring-${getColor(props.color, 20)}
					rounded-full transition-colors relative overflow-hidden
				`}
				onClick={(event) => {
					if (isLoading) return

					doBubble(event.currentTarget, event)
					dispatch(null)

					if (props.handleClick) props.handleClick()
				}}
			>
				<div class={`transition-transform ${isLoading ? 'scale-75' : ''}`}>
					<IconRender name={props.name} color={props.color} size={props.size} />
				</div>

				{isLoading && (
					<div class='absolute inset-0'>
						<Spinner color={props.color} size={size} />
					</div>
				)}
			</button>
		</Tooltip>
	)
}
