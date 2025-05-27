import type { Element } from '~/element_type'

export type IncomingMessage = {
	id: string
	syncUrl: string | null
	assetBaseUrl: string
	state: Element
}

export type OutgoingMessage = {
	id: string
	html: string
}
