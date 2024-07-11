import { ActionBlocker, ProvideDisabledContext, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function ActionBlockRender(props: ActionBlocker) {
	return (
		<ProvideDisabledContext isDisabled={props.block}>
			{props.body && <ComponentRender {...props.body} />}
		</ProvideDisabledContext>
	)
}
