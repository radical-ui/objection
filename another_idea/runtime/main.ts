import { getRootUi, UpdateAction, WindowRenderer } from 'runtime'

const sessionId = document.body.getAttribute('data-session-id')
if (!sessionId) throw new Error('Expected a data-session-id attribute')

const renderer = new WindowRenderer()
	.setActionHandler(async (upstreamActionTree) => {
		const headers = new Headers({ 'content-type': 'application/json', 'x-session-id': sessionId })
		const response = await fetch(location.href, { method: 'PUT', headers, body: JSON.stringify(upstreamActionTree) }).catch(() => null)

		if (!response) {
			renderer.notice({ message: 'You appear to be offline.', style: 'Error' })
			return
		}

		if (!response.ok) throw new Error(await response.text())

		const downstreamActions = await response.json() as UpdateAction[]

		for (const action of downstreamActions) {
			if (action.strategy === 'AddNotice') renderer.notice(action.data)
			if (action.strategy === 'FullUpdate') renderer.render(action.data)
			if (action.strategy === 'ComponentUpdate') renderer.applyUpdate(action.data[0], action.data[1])
		}
	})

renderer.render(getRootUi())
