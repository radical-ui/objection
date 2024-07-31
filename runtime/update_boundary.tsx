import { useAction } from './action.tsx'
import { ComponentRender } from './component.tsx'
import { ActionKey, Component, React } from './deps.ts'

/**
 * A boundary that allows it's children to be updated without beaming down an entirely new widget tree.
 *
 * You can optionally set a child to be displayed until an update is sent. Once an update has been sent has been sent, the original child will never be rendered. ```
 *
 * @component
 */
export interface UpdateBoundary {
	child?: Component
	action: ActionKey<Component>
}

export function UpdateBoundaryRender(props: UpdateBoundary) {
	const [currentComponent, setCurrentComponent] = React.useState(props.child)

	useAction(props.action, (component) => setCurrentComponent(component))

	if (currentComponent) return <ComponentRender {...currentComponent} />
	return <></>
}
