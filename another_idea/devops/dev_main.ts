import { getRootUi, WindowRenderer } from '../runtime/mod.ts'

const renderer = new WindowRenderer()
	.setActionHandler(async (actions) => {
		console.log('Action handled:', actions)

		await new Promise((resolve) => setTimeout(resolve, 2000))

		renderer.notice({ message: 'An event was completed', style: Math.random() > 0.5 ? 'Success' : 'Error' }, null)
	})

renderer.render(getRootUi())
