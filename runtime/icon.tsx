import { useDispatcher } from './event.tsx'
import { doBubble } from './bubble.ts'
import { EventKey, React } from './deps.ts'
import { SkeletonBlock, useSkeletonDetection } from './skeleton.tsx'
import { Spinner } from './spinner.tsx'
import { Tooltip } from './tooltip.tsx'
import { Color } from './theme.tsx'
import { getColor } from './utils.ts'

const ICON_REGISTRY_URL = '/icons'
const ICONS_CACHE = new Map<string, string>()

/**
 * TODO
 *
 * @component
 */
export interface Icon {
	name: string

	size?: number
	color?: Color
	title?: string
}

export function IconRender(props: Icon) {
	const size = props.size ?? 20
	const isSkeleton = useSkeletonDetection()
	const [svg, setSvg] = React.useState<string | null>(null)

	React.useEffect(() => {
		const cachedSvg = ICONS_CACHE.get(props.name)
		if (cachedSvg) return setSvg(cachedSvg)

		setSvg(null)
		let didStop = false

		fetch(`${ICON_REGISTRY_URL}/${props.name}.svg`)
			.then(async (res) => {
				if (!res.ok) {
					console.error(`Failed to load icon ${props.name}. Recieved HTTP status ${res.status}`)
					return null
				}

				return await res.text()
			})
			.then((svg) => {
				if (!didStop) setSvg(svg)

				if (svg) ICONS_CACHE.set(props.name, svg)
			})

		return () => {
			didStop = true
		}
	}, [props.name])

	if (isSkeleton) return <SkeletonBlock width={size} height={size} rounding='slight' />

	const inner = (
		<div
			class={`w-${props.size} h-${props.size} ${props.color ? `text-${getColor(props.color)}` : ''}`}
			dangerouslySetInnerHTML={{ __html: svg ?? '' }}
		/>
	)

	if (props.title) return <Tooltip text={props.title} offset={0}>{inner}</Tooltip>

	return inner
}

/**
 * TODO
 *
 * @component
 */
export interface IconButton {
	name: string

	size?: number
	event?: EventKey<null>
	color?: Color
	title?: string
}

export function IconButtonRender(props: IconButton & { handleClick?(): void }) {
	const color = props.color || { type: 'Fore', def: 30 }
	const size = props.size ?? 20

	const { isLoading, dispatch, isDisabled: isActionDisabled } = useDispatcher(props.event ?? null)

	const isDisabled = isActionDisabled || isLoading
	const sizeRounded = Math.round(size * 1.5)

	const backgroundStyle = `bg-fore-0 hover:bg-${getColor(color, 5)} focus:bg-${getColor(color, 10)}`

	return (
		// set the tooltip right at the edge of the icon
		<Tooltip text={props.title ?? null} offset={size * -0.25}>
			<button
				disabled={isDisabled}
				class={`
					transition-def ${isDisabled ? 'cursor-not-allowed def-50' : backgroundStyle}
					w-${sizeRounded} h-${sizeRounded} flex items-center justify-center relative
					focus:ring-4 ring-${getColor(color, 20)}
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
						<Spinner color={color} size={sizeRounded} />
					</div>
				)}
			</button>
		</Tooltip>
	)
}
