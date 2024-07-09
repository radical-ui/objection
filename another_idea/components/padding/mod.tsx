import { Padding, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function PaddingRender(props: Padding) {
	return (
		<div class={`w-full h-full pt-${props.top} pl-${props.left} pr-${props.right} pb-${props.bottom}`}>
			{props.body ? <ComponentRender {...props.body} /> : <></>}
		</div>
	)
}
