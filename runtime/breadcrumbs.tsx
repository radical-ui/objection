import { React } from './deps.ts'
import { LabelRender } from './label.tsx'
import { IconRender } from './icon.tsx'
import { Component, ComponentRender } from './component.tsx'
import { FlatLoader } from './flat_loader.tsx'
import { EventKey, useDispatcher } from './event.tsx'

/**
 * TODO
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, Bin, }
 *
 * Breadcrumbs::new() .crumb(Event::Foo, "Hi") .crumb(Event::Bar, "Bye") .crumb(Event::Bin, "Bock") .current("This") .body("Some Body") ```
 *
 * @component
 */
export interface Breadcrumbs {
	body?: Component
	crumbs: [EventKey, string][]
	current?: string
}

export function BreadcrumbsRender(props: Breadcrumbs) {
	return (
		<div class='h-full flex flex-col gap-10'>
			<div class='flex gap-5 items-center'>
				{props.crumbs.map((crumb) => <Crumb event={crumb[0]} text={crumb[1]} />)}

				{props.current && (
					<LabelRender color={{ type: 'Fore', opacity: 50 }} is_bold={true} is_italic={false} text={props.current} />
				)}
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
	event: EventKey
	text: string
}

function Crumb(props: CrumbProps) {
	const { isLoading, dispatch, isDisabled } = useDispatcher(props.event)

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
