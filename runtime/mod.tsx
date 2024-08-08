import { ComponentRender } from './component.tsx'
import { Component, React } from './deps.ts'

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

export function createStarter() {
	const rootElement = document.getElementById('root')
	if (!rootElement) throw new Error('Expected to find a #root element')

	let componentSetter = (component: Component) => {
		console.error('Not set:', component)
	}

	const StartComponent = () => {
		const [component, setComponent] = React.useState<Component | null>(null)

		React.useEffect(() => {
			componentSetter = (component) => setComponent(component)

			return () => componentSetter = () => console.error('Component setter is not set')
		}, [])

		if (component) return <ComponentRender {...component} />
		return <></>
	}

	React.render(<StartComponent />, rootElement)

	return (component: Component) => componentSetter(component)
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
