import { useDispatcher } from './action.tsx'
import { doBubble } from './bubble.ts'
import { React } from './deps.ts'
import { SkeletonBlock, useSkeletonDetection } from './skeleton.tsx'
import { Spinner } from './spinner.tsx'
import { Tooltip } from './tooltip.tsx'
import { ActionKey } from './types.ts'
import { Color } from './types.ts'
import { getColor } from './utils.ts'

const ICON_REGISTRY_URL = '/icons'
const ICONS_CACHE = new Map<string, string>()

export type IconName = string

/**
 * TODO
 *
 * **Example**
 *
 * ```rust Flex::new(FlexKind::Row) .gap(30) .justify(FlexJustify::Center) .align(FlexAlign::Center) .auto_item( Icon::new("mdi-ab-testing", 30).color(Color::Primary(100))) .auto_item( Icon::new("mdi-account-arrow-left", 30).color(Color::Success(100))) .auto_item( Icon::new("mdi-access-point", 30).color(Color::Danger(50))) ```
 *
 * @component
 */
export interface Icon {
	color: Color
	name: IconName
	size: number
	title?: string
}

export function IconRender(props: Icon) {
	const isSkeleton = useSkeletonDetection()
	const [svg, setSvg] = React.useState<string | null>(null)

	React.useEffect(() => {
		const cachedSvg = ICONS_CACHE.get(props.name)
		if (cachedSvg) return setSvg(cachedSvg)

		setSvg(null)
		let didStop = false

		fetch(`${ICON_REGISTRY_URL}/${props.name}.svg`).then((res) => res.text()).then((svg) => {
			if (!didStop) setSvg(svg)

			ICONS_CACHE.set(props.name, svg)
		})

		return () => {
			didStop = true
		}
	}, [props.name])

	if (isSkeleton) return <SkeletonBlock width={props.size} height={props.size} rounding='slight' />

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
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo }
 *
 * Flex::new(FlexKind::Row) .gap(20) .auto_item( IconButton::new("mdi-ab-testing") .color(Color::Primary(100)) .title("A description of what this does and it is a rather long description") .size(40) .action(Event::Foo) ) .auto_item(IconButton::new("mdi-ab-testing")) .auto_item( IconButton::new("mdi-ab-testing") .color(Color::Primary(100)) .action(Event::Foo) ) ```
 *
 * @component
 */
export interface IconButton {
	action?: ActionKey
	color: Color
	name: IconName
	size: number
	title?: string
}

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
