import { Center, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function CenterRender(props: Center) {
	return (
		<div class='w-full h-full flex items-center justify-center'>
			<div>
				{props.body && <ComponentRender {...props.body} />}
			</div>
		</div>
	)
}
