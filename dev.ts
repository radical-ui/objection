import { build } from './cli/build'
import { watch } from 'fs'

export const reloadScript = `
const socket = new WebSocket(\`ws://\${location.host}/_livereload.ws\`)

socket.onopen = () => console.log('Connected...')
socket.message = () => {
	location.reload()
}`

const htmlTemplate = `
<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<link rel="stylesheet" href="/_client.css">
	<script src="/_client.js" defer></script>
	<script>${reloadScript}</script>
</head>
<body></body>
</html>`

const buildClient = async () => {
	console.log('building standalne client...')
	return await build({ mode: 'standalone', sdkPath: '.', emitCss: true })
}

const openSockets = new Set<Bun.ServerWebSocket<unknown>>()

let currentClient = await buildClient()

let timeout: NodeJS.Timeout | undefined
watch('.', { recursive: true }, (_, filename) => {
	console.log(`${filename} changed...`)

	clearTimeout(timeout)
	timeout = setTimeout(async () => {
		currentClient = await buildClient()

		for (const socket of openSockets) {
			console.log(`reloading ${openSockets.size} active client(s)`)
			socket.send('reload')
		}
	}, 500)
})

Bun.serve({
	websocket: {
		open(socket) {
			openSockets.add(socket)
		},
		message() {},
	},
	routes: {
		'/_livereload.ws': (request, server) => {
			if (server.upgrade(request)) return undefined as never // just a hack to make TS happy

			return new Response('Upgrade failed')
		},
		'/_client.js': () => {
			return new Response(currentClient.code, { headers: { 'Content-Type': 'application/javascript' } })
		},
		'/_client.css': () => {
			return new Response(currentClient.css, { headers: { 'Content-Type': 'text/css' } })
		},
		'/*': () => {
			return new Response(htmlTemplate, { headers: { 'Content-Type': 'text/html' } })
		},
	},
})

console.log('dev server listening at http://localhost:3000')
