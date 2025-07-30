import type { Element } from '~/element_type'

export type DownstreamMessage = RootDownstreamMessage | SlotDownstreamMessage | MetaDownstreamMessage

export type RootDownstreamMessage = {
	kind: 'root'
	element: Element
}

export type SlotDownstreamMessage = {
	kind: 'slot'
	slotId: string
	element: Element
}

export type MetaDownstreamMessage = {
	kind: 'meta'
	meta: WindowMeta
}

export type WindowMeta = {
	title: string
	description: string
	iconUrl: string
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
