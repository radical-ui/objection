import { Component, ComponentRender } from './component.tsx'
import { React } from './deps.ts'
import { GlobalCss } from './utils.ts'

const css = new GlobalCss(`
	.scrollable-box {
		scrollbar-color: rgba(127, 127, 127, 0.5) transparent;
		scrollbar-width: thin;
	}
`)

/**
 * A scrollable box.
 *
 * **Example**
 *
 * ```rust ScrollableBox::new().body(Padding::all(20).body("Contrary to popular belief, Lorem Ipsum is not simply random text. It has roots in a piece of classical Latin literature from 45 BC, making it over 2000 years old. Richard McClintock, a Latin professor at Hampden-Sydney College in Virginia, looked up one of the more obscure Latin words, consectetur, from a Lorem Ipsum passage, and going through the cites of the word in classical literature, discovered the undoubtable source. Lorem Ipsum comes from sections 1.10.32 and 1.10.33 of \"de Finibus Bonorum et Malorum\" (The Extremes of Good and Evil) by Cicero, written in 45 BC. This book is a treatise on the theory of ethics, very popular during the Renaissance. The first line of Lorem Ipsum, \"Lorem ipsum dolor sit amet..\", comes from a line in section 1.10.32. But I must explain to you how all this mistaken idea of denouncing pleasure and praising pain was born and I will give you a complete account of the system, and expound the actual teachings of the great explorer of the truth, the master-builder of human happiness. No one rejects, dislikes, or avoids pleasure itself, because it is pleasure, but because those who do not know how to pursue pleasure rationally encounter consequences that are extremely painful. Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain, but because occasionally circumstances occur in which toil and pain can procure him some great pleasure. To take a trivial example, which of us ever undertakes laborious physical exercise, except to obtain some advantage from it? But who has any right to find fault with a man who chooses to enjoy a pleasure that has no annoying consequences, or one who avoids a pain that produces no resultant pleasure? On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.")) ```
 *
 * @component
 */
export interface ScrollableBox {
	body?: Component
}

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
