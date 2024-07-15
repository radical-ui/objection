import { getColor, Icon, React, Tooltip } from 'runtime'
import { SkeletonBlock, useSkeletonDetection } from '../skeleton/mod.tsx'

const ICON_REGISTRY_URL = 'https://localhost:7765/icons'
const ICONS_CACHE = new Map<string, string>()

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
