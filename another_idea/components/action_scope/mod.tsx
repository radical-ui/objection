import { ActionScope, ProvideScope, React, useActionScope } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function ActionScopeRender(props: ActionScope) {
	const parentScope = useActionScope()

	return (
		<div class='w-full h-full'>
			<ProvideScope scope={[{ key: props.scope, payload: props.payload ?? null }, ...parentScope]}>
				{props.body ? <ComponentRender {...props.body} /> : <></>}
			</ProvideScope>
		</div>
	)
}
