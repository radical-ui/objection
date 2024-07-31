import { ComponentIndexRenderer, ComponentRender, ProvideComponentIndexRenderer } from './component.tsx'
import { Component, MOUNT_ACTION, React, READY_EVENT, sendEvent, setEndpoint, shouldSendReadyEvent } from './deps.ts'
import { UpdateBoundaryRender } from './update_boundary.tsx'

export type { ActionKey, AnyEvent, Component, EventKey } from './deps.ts'

export * from './event.tsx'
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
export * from './notice.tsx'
export * from './theme.tsx'
export * from './title.tsx'

export function start(syncUrl: URL, initialComponent: Component, componentRenderer: ComponentIndexRenderer) {
	setEndpoint(syncUrl)

	const rootElement = document.getElementById('root')
	if (!rootElement) throw new Error('Expected to find a #root element')

	React.render(
		<ProvideComponentIndexRenderer renderer={componentRenderer}>
			<UpdateBoundaryRender child={<ComponentRender {...initialComponent} />} action={MOUNT_ACTION} />
		</ProvideComponentIndexRenderer>,
		rootElement,
	)

	if (shouldSendReadyEvent()) {
		console.log('Sending ready event...')

		sendEvent(READY_EVENT, {
			token: localStorage.getItem('token'),
		})
			.then(() => {
				console.log('Mounted.')
			})
	}
}

/**
 * A "nothing" component. Renders nothing.
 * @component
 */
// deno-lint-ignore no-empty-interface
export interface Fragment {}

export function FragmentRender() {
	return <></>
}
