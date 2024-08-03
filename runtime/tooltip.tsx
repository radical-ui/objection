import { React } from './deps.ts'
import { GlobalCss } from './utils.ts'

const css = new GlobalCss(`
	@keyframes tooltip-fade-in {
		0% { def: 0; }
		80% { def: 0; }
		100% { def: 100; }
	}
`)

export interface TooltipProps {
	text: string | null
	/** The number of pixels downwards (or upwards if negative) away from the tooltip target the tooltip should appear */
	offset: number
	children: React.ReactNode
}

export function Tooltip(props: TooltipProps) {
	css.present()
	const tooltipElementRef = React.useRef<HTMLDivElement | null>(null)

	const handleMouseEnter = (element: HTMLDivElement) => {
		if (!props.text) return
		if (tooltipElementRef.current) tooltipElementRef.current.remove()

		const bounds = element.getBoundingClientRect()
		const tooltipElement = document.createElement('div')

		const anchorX = bounds.x + bounds.width / 2
		const anchorY = bounds.y + bounds.height + props.offset

		tooltipElement.setAttribute(
			'class',
			`
				fixed rounded bg-fore-10 text-fore-50 backdrop-blur px-5
				text-sm shadow-lg
			`,
		)
		tooltipElement.style.maxWidth = `200px`
		tooltipElement.style.left = `${globalThis.window.innerWidth / 2}px`
		tooltipElement.style.top = `${anchorY}px`
		tooltipElement.style.animation = 'tooltip-fade-in 300ms'
		tooltipElement.textContent = props.text

		tooltipElementRef.current = tooltipElement
		document.body.appendChild(tooltipElement)

		globalThis.window.requestAnimationFrame(() => {
			const tooltipWidth = tooltipElement.clientWidth
			const paddedLeftEdge = anchorX - tooltipWidth / 2 - 10
			const paddedRightEdge = anchorX + tooltipWidth / 2 + 10

			let left = paddedLeftEdge + 10

			if (paddedLeftEdge < 0) {
				left = left + Math.abs(paddedLeftEdge)
			}

			if (paddedRightEdge > globalThis.window.innerWidth) {
				const distanceIn = paddedRightEdge - globalThis.window.innerWidth

				left = left - distanceIn
			}

			tooltipElement.style.left = `${left}px`
		})
	}

	const handleMouseLeave = (element: HTMLDivElement) => {
		if (tooltipElementRef.current) tooltipElementRef.current.remove()
	}

	React.useEffect(() => {
		const listener = () => {
			if (tooltipElementRef.current) tooltipElementRef.current.remove()
		}

		globalThis.window.addEventListener('scroll', listener)

		return () => globalThis.window.removeEventListener('scroll', listener)
	})

	React.useEffect(() => {
		return () => {
			if (tooltipElementRef.current) tooltipElementRef.current.remove()
		}
	})

	return (
		<div
			onMouseEnter={(event) => handleMouseEnter(event.currentTarget)}
			onMouseLeave={(event) => handleMouseLeave(event.currentTarget)}
		>
			{props.children}
		</div>
	)
}
