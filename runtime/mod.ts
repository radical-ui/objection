import { Component, UpdateAction } from './types.ts'
import { WindowRenderer } from './window_renderer.tsx'

// TODO these should be integrated into the actual components
export * from './types.ts'

export * from './action_blocker/mod.tsx'
export * from './action_scope/mod.tsx'
export * from './breadcrumbs/mod.tsx'
export * from './button/mod.tsx'
export * from './card/mod.tsx'
export * from './center/mod.tsx'
export * from './center_layout/mod.tsx'
export * from './checkbox_input/mod.tsx'
export * from './circle_progress/mod.tsx'
export * from './divider/mod.tsx'
export * from './flex/mod.tsx'
export * from './fragment/mod.tsx'
export * from './header/mod.tsx'
export * from './icon/mod.tsx'
export * from './icon_button/mod.tsx'
export * from './image/mod.tsx'
export * from './label/mod.tsx'
export * from './modal/mod.tsx'
export * from './nested_flow/mod.tsx'
export * from './padding/mod.tsx'
export * from './preview_box/mod.tsx'
export * from './radio_input/mod.tsx'
export * from './scrollable_box/mod.tsx'
export * from './sidebar_layout/mod.tsx'
export * from './skeleton/mod.tsx'
export * from './table/mod.tsx'
export * from './text_input/mod.tsx'
export * from './update_boundary/mod.tsx'

export function start(syncUrl: string, initialComponent: Component, componentRenderer: React.FC<Component>) {
	const sessionId = crypto.randomUUID()

	const renderer = new WindowRenderer()
		.setActionHandler(async (upstreamActionTree) => {
			const headers = new Headers({ 'content-type': 'application/json', 'x-session-id': sessionId })
			const response = await fetch(syncUrl, { method: 'PUT', headers, body: JSON.stringify(upstreamActionTree) }).catch(() => null)

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

	renderer.render(initialComponent)
}
