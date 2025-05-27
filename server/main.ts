import type { Element } from '~/element_type'
import { WorkerCache } from './worker_cache'

type TransformRequest = {
	syncUrl: string
	state: Element
}

const ASSET_BASE_URL = process.env.ASSET_BASE_URL
if (!ASSET_BASE_URL) throw new Error('No ASSET_BASE_URL env var configured')

const cache = new WorkerCache(ASSET_BASE_URL)

const server = Bun.serve({
	routes: {
		'/:version/transform': {
			async POST(request) {
				const body: TransformRequest = await request.json()
				const version = request.params.version
				const ip = server.requestIP(request)

				if (!body.syncUrl || typeof body.syncUrl !== 'string') {
					console.info('recieved invalid syncUrl param', { ip })
					return new Response('Invalid `syncUrl` param in request body', { status: 400 })
				}
				if (!body.state || typeof body.state !== 'object') {
					console.info('recieved invalid state param', { ip, syncUrl: body.syncUrl })
					return new Response('Invalid `state` param in request body', { status: 400 })
				}

				console.info('running worker', { ip, version, syncUrl: body.syncUrl })
				const html = await cache.run(version, body.syncUrl, body.state)

				return new Response(html, { headers: { 'Content-Type': 'text/html' } })
			},
		},
	},
	fetch(request) {
		console.info('recieved unmatched route', { ip: server.requestIP(request) })
		return new Response('Route not found', { status: 404 })
	},
	error(error) {
		console.error(error)
		return new Response('An internal error has occurred', { status: 500 })
	},
})

console.log(`Listening at http://${server.hostname}:${server.port}`)
