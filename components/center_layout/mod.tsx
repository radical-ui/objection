import { CenterLayout, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'
import { LabelRender } from '../label/mod.tsx'

export function CenterLayoutRender(props: CenterLayout) {
	return (
		<div class='flex items-center flex-col w-full h-full p-30' style={{ justifyContent: 'safe center' }}>
			<div class={`flex flex-col gap-20 ${props.thin ? 'max-w-sm' : 'max-w-xl'} w-full`}>
				<h1 class='text-3xl'>
					<LabelRender color={{ type: 'Fore', opacity: 80 }} is_bold is_italic={false} text={props.title} />
				</h1>
				{props.subtitle && <h3 class='text-fore-50'>{props.subtitle}</h3>}
				{props.body && (
					<div>
						<ComponentRender {...props.body} />
					</div>
				)}
			</div>
		</div>
	)
}
