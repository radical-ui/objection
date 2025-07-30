import { OwnedObservable, type Observable } from '@emooring/observable'
import type { Element } from '../element_type'
import type { DownstreamMessage, WindowMeta } from './message'
import { reconcileSlot } from './reconcile_slot'

const REQUEST_RETRY_INTERVAL = 500
const CONNECTION_RETRY_INTERVAL = 1000

export class SyncTool {
	#url: string
	#rootElementObservable: OwnedObservable<Element>
	#windowMetaObservable: OwnedObservable<WindowMeta>

	constructor(url: string, initialState: Element, initialWindowMeta: WindowMeta) {
		this.#url = url
		this.#rootElementObservable = new OwnedObservable(initialState)
		this.#windowMetaObservable = new OwnedObservable(initialWindowMeta)
	}

	async start() {
		while (true) {
			const response = await this.#getResponse()
			const contentType = response.headers.get('Content-Type')
			const rawReconnectDelay = response.headers.get('Rui-Reconnect-Delay')
			const reconnectDelay = rawReconnectDelay ? parseInt(rawReconnectDelay) : undefined

			if (contentType !== 'application/jsonl') {
				console.warn('syncURL response was not application/jsonl. Going to try to parse as jsonl anyways.')
			}

			if (!response.body) return console.error('No response body was recieved')

			await this.#streamBody(response.body)

			if (reconnectDelay === undefined) return

			console.log(`connection finished. Waiting ${reconnectDelay}ms before reconnecting...`)
			await new Promise(resolve => setTimeout(resolve, reconnectDelay))
		}
	}

	observeRoot(): Observable<Element> {
		return this.#rootElementObservable
	}

	async #getResponse() {
		while (true) {
			try {
				return await fetch(this.#url)
			} catch (error) {
				console.error(`error fetching response. Retrying in ${REQUEST_RETRY_INTERVAL}ms`, error)
			}

			await new Promise(resolve => setTimeout(resolve, REQUEST_RETRY_INTERVAL))
		}
	}

	async #streamBody(stream: ReadableStream<Uint8Array>) {
		const reader = stream.getReader()
		const decoder = new TextDecoder()

		let currentStreamResult = await reader.read()
		let currentChunk = ''

		while (!currentStreamResult.done) {
			const chunk = decoder.decode(currentStreamResult.value)
			currentChunk += chunk

			const newlineIndex = currentChunk.indexOf('\n')
			if (newlineIndex !== -1) {
				const line = currentChunk.slice(0, newlineIndex)
				currentChunk = currentChunk.slice(newlineIndex + 1)

				this.#handleMessage(line)
			}

			currentStreamResult = await reader.read()
		}
	}

	async #retryConnection() {
		console.info('retrying connection in 1 second ...')
		await new Promise(resolve => setTimeout(resolve, CONNECTION_RETRY_INTERVAL))

		this.start()
	}

	async #handleMessage(text: string) {
		let json
		try {
			json = JSON.parse(text)
		} catch (error) {
			return console.error('error parsing recieved chunk:', error, { chunk: text })
		}

		this.#processMessage(json)
	}

	#processMessage(message: DownstreamMessage) {
		if (message.kind === 'meta') return this.#windowMetaObservable.set(message.meta)
		if (message.kind === 'root') return this.#rootElementObservable.set(message.element)
		if (message.kind === 'slot') {
			const value = this.#rootElementObservable.get()
			reconcileSlot(this.#rootElementObservable.get(), message.slotId, message.element)

			return this.#rootElementObservable.set(value)
		}

		throw new Error('unknown message kind')
	}
}
