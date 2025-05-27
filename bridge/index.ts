import type { Element } from '~/element_type'

export * from './hydration_state'
export * as SsrWorker from './worker'

export type DownstreamMessage = WindowMessage | SetChildMessage

export type WindowMessage = {
	type: 'window'
	id: string
	root: Element
}

export type SetChildMessage = {
	type: 'set_child'
	boxId: string
	child: Element
}

export type UpstreamMessage = FormSubmissionMessage | SubmissionMessage

export type FormSubmissionMessage = {
	type: 'form_submission'
	key: string
	data: Record<string, ActionData>
}

export type SubmissionMessage = {
	type: 'submission'
	key: string
	data: ActionData
}

export type ActionData = string | number | boolean

export function getSyncUrl() {
	const syncUrl = document.body.getAttribute('sync-url')
	if (!syncUrl) return null

	return new URL(syncUrl)
}
