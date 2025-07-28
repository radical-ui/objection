import { renderList } from './list'

const port = process.env.PORT || 8000

Bun.serve({
	port,
	routes: {
		'/': request => {
			const observe = request.headers.get('Accept') === 'application/jsonl'
			const initialData = renderList()

			const stream = new ReadableStream<string>({
				start(controller) {
					controller.enqueue(JSON.stringify(initialData) + '\n')
					if (!observe) controller.close()
				},
			})

			return new Response(stream, {
				headers: { 'Content-Type': observe ? 'application/jsonl' : 'application/json' },
			})
		},
	},
})

console.log(`listening at http://localhost:${port}`)
