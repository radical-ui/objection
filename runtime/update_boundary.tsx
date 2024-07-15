import { Component, ComponentRender } from './component.tsx'
import { useUpdates } from './component_update.tsx'
import { React } from './deps.ts'

/**
 * A boundary that allows it's children to be updated without beaming down a new window.
 *
 * You can optionally set a child to be displayed until an update is sent. Once an update has been sent has been sent, the original child will never be rendered. ```
 *
 * @component
 */
export interface UpdateBoundary {
	child?: Component
	id: number
}

export function UpdateBoundaryRender(props: UpdateBoundary) {
	const update = useUpdates<Component>(props.id)
	const component = update ?? props.child ?? null

	if (component) return <ComponentRender {...component} />
	return <></>
}
