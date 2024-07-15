import { GlobalCss, React, ScrollableBox } from 'runtime'
import { ComponentRender } from '../mod.tsx'

const css = new GlobalCss(`
	.scrollable-box {
		scrollbar-color: rgba(127, 127, 127, 0.5) transparent;
		scrollbar-width: thin;
	}
`)

export function ScrollableBoxRender(props: ScrollableBox) {
	css.present()

	const elementRef = React.useRef<HTMLDivElement | null>(null)
	const [fadeTop, setFadeTop] = React.useState(false)
	const [fadeBottom, setFadeBottom] = React.useState(false)

	const fadeIfNecessary = () => {
		if (!elementRef.current) return
		const element = elementRef.current

		const topEdge = element.scrollTop
		const bottomEdge = topEdge + element.clientHeight

		const isNearTop = topEdge <= 5
		const isNearBottom = bottomEdge >= element.scrollHeight - 5

		setFadeTop(!isNearTop)
		setFadeBottom(!isNearBottom)
	}

	const mask = `linear-gradient(
		to bottom,
		transparent,
		black ${fadeTop ? 50 : 0}px,
		black calc(100% - ${fadeBottom ? 50 : 0}px),
		transparent 100%
	)`

	React.useEffect(() => {
		fadeIfNecessary()
	})

	return (
		<div class='w-full h-full' style={{ maskImage: mask, webkitMaskImage: mask }}>
			<div ref={elementRef} class='h-full w-full overflow-auto scrollable-box' onScroll={() => fadeIfNecessary()}>
				<div class='w-full'>
					{props.body && <ComponentRender {...props.body} />}
				</div>
			</div>
		</div>
	)
}
