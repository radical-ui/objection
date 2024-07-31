import { Component, React } from './deps.ts'

export type ComponentIndexRenderer = (component: Component) => React.ReactElement

const Context = React.createContext<ComponentIndexRenderer | null>(null)

export interface ProvideComponentIndexRendererProps {
	renderer: ComponentIndexRenderer
	children: React.ReactNode
}

export function ProvideComponentIndexRenderer(props: ProvideComponentIndexRendererProps) {
	return <Context.Provider value={props.renderer}>{props.children}</Context.Provider>
}

export function ComponentRender(component: Component) {
	const renderer = React.useContext(Context)

	if (!renderer) {
		throw new Error(
			'No component index renderer was found in component hierarchy. <ProvideComponentIndexRenderer> must be above any instances of <Component>',
		)
	}

	return renderer(component)
}
