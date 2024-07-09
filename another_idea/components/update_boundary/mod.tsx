import { Component, React, UpdateBoundary, useUpdates } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function UpdateBoundaryRender(props: UpdateBoundary) {
	const update = useUpdates<Component>(props.id)
	const component = update ?? props.child ?? null

	if (component) return <ComponentRender {...component} />
	return <></>
}
