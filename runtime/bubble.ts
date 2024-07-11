import { GlobalCss } from './utils.ts'

const bubbleCss = new GlobalCss(`
	.bubble {
		width: 5px;
		height: 5px;
		background: currentColor;
		animation: bubble 0.8s forwards;
		border-radius: 50%;
		pointer-events: none;
	}

	@keyframes bubble {
		from {
			width: 0px;
			height: 0px;
			opacity: 0.5;
		}
		to {
			width: 200px;
			height: 200px;
			transform: translate(-100px, -100px);
			opacity: 0;
		}
	}
`)

export function doBubble(element: HTMLElement, event: MouseEvent) {
	bubbleCss.present()

	const bubble = document.createElement('div')
	bubble.classList.add('bubble')

	const parentBounds = element.getBoundingClientRect()
	const xInEl = event.clientX - parentBounds.x
	const yInEl = event.clientY - parentBounds.y

	bubble.style.position = 'absolute'
	bubble.style.top = `${yInEl}px`
	bubble.style.left = `${xInEl}px`

	element.appendChild(bubble)

	setTimeout(() => {
		bubble.remove()
	}, 1000)
}
