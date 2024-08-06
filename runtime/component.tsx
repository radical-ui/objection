import { Component, getComponentRenderer, React } from './deps.ts'

export function ComponentRender(component: Component) {
	const { func: Func, params } = getComponentRenderer(component)

	// @ts-ignore runtime_lib is intentionally runtime-agnostic
	return <Func {...params} />
}
