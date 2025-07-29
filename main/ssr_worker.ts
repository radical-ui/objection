import { render } from 'svelte/server'
import { generateHydrationStateHtml, type SsrWorker } from '~/bridge'

import Root from '../root.svelte'

declare const self: Worker

self.onmessage = event => {
	const incomingMessage = event.data as SsrWorker.WorkerIncomingMessage
	const syncUrl = incomingMessage.syncUrl ? new URL(incomingMessage.syncUrl) : null

	const result = render(Root, {
		props: { syncUrl, state: incomingMessage.state },
	})

	const baseHead = `<meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">`
	const cssLink = `<link href="${incomingMessage.assetBaseUrl}/combined.css" rel="stylesheet">`
	const jsLink = `<script async defer src="${incomingMessage.assetBaseUrl}/bundle.js"></script>`
	const head = `<head>${baseHead}${cssLink}${result.head}${jsLink}${generateHydrationStateHtml(incomingMessage.state)}</head>`
	const body = `<body sync-url="${incomingMessage.syncUrl}">${result.body}</body>`
	const html = `<!DOCTYPE html><html>${head}${body}</html>`

	const outgoingMessage: SsrWorker.WorkerOutgoingMessage = {
		id: incomingMessage.id,
		html,
	}

	self.postMessage(outgoingMessage)
}
