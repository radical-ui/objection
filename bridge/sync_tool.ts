import { OwnedObservable, type Observable } from '@emooring/observable'
import type { Element } from '../element_type'
import type { WindowMeta } from './message'

const REQUEST_RETRY_INTERVAL = 500
const CONNECTION_RETRY_INTERVAL = 1000

export class SyncTool {
	#url: string
	#observable: OwnedObservable<Element>
	#windowMetaObservable: OwnedObservable<WindowMeta>

	constructor(url: string, initialState: Element, initialWindowMeta: WindowMeta) {
		this.#url = url
		this.#observable = new OwnedObservable(initialState)
		this.#windowMetaObservable = new OwnedObservable(initialWindowMeta)
	}

	async start() {
		const response = await this.#getResponse()
		const contentType = response.headers.get('Content-Type')

		if (contentType !== 'application/jsonl') {
			console.warn('syncURL response was not application/json or application/jsonl. Going to try to parse as json anyways.')
		}

		if (!response.body) {
			return console.error('No response body was recieved')
		}
		return await this.#readStreamedResponse(response.body)
	}

	observable(): Observable<Element> {
		return this.#observable
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

	async #readDirectResponse(response: Response) {
		const element = await response.json()
	}

	async #readStreamedResponse(stream: ReadableStream<Uint8Array>) {
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

				this.#emitChunk(line)
			}

			currentStreamResult = await reader.read()
		}
	}

	async #retryConnection() {
		console.info('retrying connection in 1 second ...')
		await new Promise(resolve => setTimeout(resolve, CONNECTION_RETRY_INTERVAL))

		this.start()
	}

	async #emitChunk(text: string) {
		let json
		try {
			json = JSON.parse(text)
		} catch (error) {
			return console.error('error parsing recieved chunk:', error, { chunk: text })
		}

		if (typeof json.$ !== 'string') {
			return console.error('recieved value is not a chunk')
		}
	}
}
