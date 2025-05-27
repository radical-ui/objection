import type { SsrWorker } from '~/bridge'
import type { Element } from '~/element_type'

export class WorkerCache {
	#baseUrl: string
	#htmlListeners = new Map<string, (html: string) => void>()
	#workers = new Map<string, Promise<Worker>>()

	constructor(baseUrl: string) {
		this.#baseUrl = baseUrl
	}

	async run(version: string, syncUrl: string, state: Element) {
		const worker = await this.#get(version)
		const id = crypto.randomUUID()

		const htmlPromise = new Promise<string>(resolve => {
			this.#htmlListeners.set(id, resolve)
		})

		const message: SsrWorker.IncomingMessage = {
			id,
			assetBaseUrl: `${this.#baseUrl}/${version}`,
			syncUrl,
			state,
		}

		console.info('sent event to worker', { version, id })
		worker.postMessage(message)

		return await htmlPromise
	}

	async #get(version: string) {
		const cachedWorker = this.#workers.get(version)
		if (cachedWorker) return cachedWorker

		const newWorker = this.#fetchAndRegisterWorker(version)
		this.#workers.set(version, newWorker)

		return await newWorker
	}

	async #fetchAndRegisterWorker(version: string) {
		const url = `${this.#baseUrl}/${version}/ssr_worker.js`

		const blob = await fetch(url).then(res => res.blob())
		console.info('downloading worker', { url, version })

		const object = URL.createObjectURL(blob)
		const worker = new Worker(object)

		worker.onmessage = (event: MessageEvent<SsrWorker.OutgoingMessage>) => {
			const { id, html } = event.data
			console.info('recieved event from worker', { version, id })

			const listener = this.#htmlListeners.get(id)
			if (!listener) return console.error('recieved html for event with no listeners', { id, version })

			this.#htmlListeners.delete(id)
			listener(html)
		}

		worker.onerror = event => {
			console.error('worker errored', event.error)
		}

		return worker
	}
}
