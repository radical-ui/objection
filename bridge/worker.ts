import type { Element } from '~/element_type'

export type WorkerIncomingMessage = {
	id: string
	syncUrl: string | null
	assetBaseUrl: string
	state: Element
}

export type WorkerOutgoingMessage = {
	id: string
	html: string
}
