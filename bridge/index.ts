import type { Element } from '~/element_type'

export * from './hydration_state'
export * as SsrWorker from './worker'

export function getSyncUrl() {
	const syncUrl = document.body.getAttribute('sync-url')
	if (!syncUrl) return null

	return new URL(syncUrl)
}
