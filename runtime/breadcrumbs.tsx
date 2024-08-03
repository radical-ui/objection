import { Component, EventKey, React } from './deps.ts'
import { LabelRender } from './label.tsx'
import { IconRender } from './icon.tsx'
import { ComponentRender } from './component.tsx'
import { FlatLoader } from './flat_loader.tsx'
import { useDispatcher } from './event.tsx'

export interface Crumb {
	event: EventKey<null>
	text: string
}

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
	crumbs?: Crumb[]
	current?: string
}

export function BreadcrumbsRender(props: Breadcrumbs) {
	const crumbs = props.crumbs || []

	return (
		<div class='h-full flex flex-col gap-10'>
			<div class='flex gap-5 items-center'>
				{crumbs.map((crumb) => <Crumb {...crumb} />)}

				{props.current && <LabelRender color={{ type: 'Fore', def: 50 }} isBold={true} isItalic={false} text={props.current} />}
			</div>

			{props.body && (
				<div class='flex-1'>
					<ComponentRender {...props.body} />
				</div>
			)}
		</div>
	)
}

function Crumb(props: Crumb) {
	const { isLoading, dispatch, isDisabled } = useDispatcher(props.event)

	return (
		<>
			<button
				disabled={isLoading || isDisabled}
				class={`relative transition-def ${isLoading ? 'def-80 cursor-not-allowed' : 'cursor-pointer hover:def-80'}`}
				onClick={() => dispatch(null)}
			>
				<LabelRender color={{ type: 'Primary', def: 100 }} isBold={true} isItalic={false} text={props.text} />

				{isLoading && (
					<div class='absolute -bottom-4 left-0 right-0 overflow-hidden'>
						<FlatLoader color={{ type: 'Primary', def: 100 }} size={4} />
					</div>
				)}
			</button>

			<IconRender color={{ type: 'Fore', def: 10 }} name='mdi-chevron-right' size={20} />
		</>
	)
}
