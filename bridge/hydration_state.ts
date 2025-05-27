import type { Element } from '~/element_type'

const key = '__UiHD' // short for UI hydration data

export function generateHydrationStateHtml(element: Element) {
	return `<script>window.${key}=${JSON.stringify(element)}</script>`
}

export function getHydrationState(): Element {
	// @ts-expect-error
	const state = globalThis.window[key] as Element
	if (!state) throw new Error('Expected to find hydration state, but none was supplied')

	return state
}
