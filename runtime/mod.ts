import { Component } from './component.tsx'
import { React } from './deps.ts'
import { UpdateAction } from './types.ts'
import { WindowRenderer } from './window_renderer.tsx'

export * from './action.tsx'
export * from './component.tsx'
export * from './breadcrumbs.tsx'
export * from './button.tsx'
export * from './card.tsx'
export * from './center.tsx'
export * from './checkbox_input.tsx'
export * from './circle_progress.tsx'
export * from './divider.tsx'
export * from './flex.tsx'
export * from './header.tsx'
export * from './icon.tsx'
export * from './image.tsx'
export * from './label.tsx'
export * from './modal.tsx'
export * from './nested_flow.tsx'
export * from './padding.tsx'
export * from './preview_box.tsx'
export * from './radio_input.tsx'
export * from './scrollable_box.tsx'
export * from './sidebar_layout.tsx'
export * from './skeleton.tsx'
export * from './table.tsx'
export * from './text_input.tsx'
export * from './update_boundary.tsx'

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

/**
 * A "nothing" component. Renders nothing.
 * @component
 */
// deno-lint-ignore no-empty-interface
export interface Fragment {}

export function FragmentRender() {
	return React.createElement(React.Fragment, {})
}
