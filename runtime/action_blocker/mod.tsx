import { ActionBlocker, ProvideDisabledContext, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function ActionBlockerRender(props: ActionBlocker) {
	return (
		<ProvideDisabledContext isDisabled={props.block}>
			{props.body && <ComponentRender {...props.body} />}
		</ProvideDisabledContext>
	)
}
