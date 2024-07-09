import { ActionKey, Breadcrumbs, FlatLoader, React, useDispatcher } from 'runtime'
import { ComponentRender } from '../mod.tsx'
import { LabelRender } from '../label/mod.tsx'
import { IconRender } from '../icon/mod.tsx'

export function BreadcrumbsRender(props: Breadcrumbs) {
	return (
		<div class='h-full flex flex-col gap-10'>
			<div class='flex gap-5 items-center'>
				{props.crumbs.map((crumb) => <Crumb action={crumb[0]} text={crumb[1]} />)}

				{props.current && <LabelRender color={{ type: 'Fore', opacity: 50 }} is_bold={true} is_italic={false} text={props.current} />}
			</div>

			{props.body && (
				<div class='flex-1'>
					<ComponentRender {...props.body} />
				</div>
			)}
		</div>
	)
}

interface CrumbProps {
	action: ActionKey
	text: string
}

function Crumb(props: CrumbProps) {
	const { isLoading, dispatch, isDisabled } = useDispatcher(props.action)

	return (
		<>
			<button
				disabled={isLoading || isDisabled}
				class={`relative transition-opacity ${isLoading ? 'opacity-80 cursor-not-allowed' : 'cursor-pointer hover:opacity-80'}`}
				onClick={() => dispatch(null)}
			>
				<LabelRender color={{ type: 'Primary', opacity: 100 }} is_bold={true} is_italic={false} text={props.text} />

				{isLoading && (
					<div class='absolute -bottom-4 left-0 right-0 overflow-hidden'>
						<FlatLoader color={{ type: 'Primary', opacity: 100 }} size={4} />
					</div>
				)}
			</button>

			<IconRender color={{ type: 'Fore', opacity: 10 }} name='mdi-chevron-right' size={20} />
		</>
	)
}
