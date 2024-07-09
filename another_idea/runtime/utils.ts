import { Color } from './types.ts'

export function getColor(color: Color, opacityOverride: number | null = null) {
	const opacity = opacityOverride !== null
		? opacityOverride < 0 ? Math.max(color.opacity + opacityOverride, 5) : opacityOverride
		: color.opacity
	const type = color.type === 'DecorationFore' ? 'decoration-fore' : color.type.toLowerCase()

	return `${type}-${opacity}`
}

export function makeDebounce<P>(fn: (param: P) => void, ms: number): (param: P) => void {
	let timer: number

	return (param) => {
		clearTimeout(timer)
		timer = setTimeout(() => fn(param), ms)
	}
}

export function isOk<T>(value: T | undefined | null): value is T {
	return value !== undefined && value !== null
}

export class GlobalCss {
	#css: string
	#didPresent = false

	constructor(css: string) {
		this.#css = css
	}

	present() {
		if (this.#didPresent) return

		const style = document.createElement('style')
		style.textContent = this.#css

		document.head.appendChild(style)
		this.#didPresent = true
	}
}

export class ManyMap<K, V> {
	#map = new Map<K, V[]>()

	add(key: K, value: V): () => void {
		const existingItems = this.#map.get(key)

		if (existingItems) existingItems.push(value)
		else this.#map.set(key, [value])

		return () => {
			const items = this.#map.get(key)
			if (!items) return

			items.splice(items.indexOf(value), 1)
		}
	}

	get(key: K): V[] {
		return this.#map.get(key) ?? []
	}
}
