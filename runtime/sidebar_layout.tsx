import { useDispatcher } from './event.tsx'
import { Component, ComponentRender } from './component.tsx'
import { EventKey, React } from './deps.ts'
import { IconName, IconRender } from './icon.tsx'
import { Image, ImageRender } from './image.tsx'
import { getEventId } from '../runtime_lib/mod.ts'

/**
 * A sidebar application layout.
 *
 * **Example**
 *
 * ```rust SidebarLayout::new("Abc Corp") ```
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Action { Foo, Bar, }
 *
 * SidebarLayout::new("Abc Corp") .title_event(Action::Foo) .logo(Image::new("https://github.githubassets.com/assets/3m-0151c2fda0ce.svg").width(30).height(30)) .event_item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .event_item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) .group( SidebarGroup::new("Main") .item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) ) .group( SidebarGroup::new("Records") .item(SidebarItem::new("Tasks").icon("mdi-ab-testing").event(Action::Foo)) .item(SidebarItem::new("Activities").icon("mdi-ab-testing").event(Action::Bar)) ) .initial_event(Action::Foo) .footer(Center::new().body("Za feetsies")) ```
 *
 * @component
 */
export interface SidebarLayout {
	eventItems: SidebarItem[]
	body?: Component
	footer?: Component
	groups: SidebarGroup[]
	initialEvent?: EventKey<null>
	logo?: Image
	title: string
	titleEvent?: EventKey<null>
}
export interface SidebarItem {
	event?: EventKey<null>
	icon?: IconName
	title: string
}
export interface SidebarGroup {
	items: SidebarItem[]
	name: string
}

export function SidebarLayoutRender(props: SidebarLayout) {
	const [selectedKey, setSelectedKey] = React.useState(props.initialEvent ? getEventId(props.initialEvent) : null)

	const setItem = (key: EventKey<null>) => setSelectedKey(getEventId(key))

	return (
		<div class='h-full flex'>
			<div class='w-300 bg-fore-5'>
				<div class='h-full p-20 flex flex-col gap-40 bg-gradient-to-tr from-secondary-15 via-primary-1 to-transparent backdrop-blur-md'>
					<button
						class='flex gap-10 items-center'
						onClick={() => {
							if (!props.titleEvent) return

							setItem(props.titleEvent)
						}}
					>
						{props.logo && <ImageRender {...props.logo} />}
						<div class='text-xl font-bold'>{props.title}</div>
					</button>

					{props.eventItems.length > 0 && (
						<div class='flex flex-col gap-10'>
							{props.eventItems.map((item) => (
								<Item
									onSelected={() => {
										if (!item.event) return

										setItem(item.event)
									}}
									isSelected={item.event ? selectedKey === getEventId(item.event) : false}
									{...item}
								/>
							))}
						</div>
					)}

					<div class='flex flex-col gap-30'>
						{props.groups.map((group) => {
							return (
								<>
									<div class='flex flex-col gap-10'>
										<div class='uppercase text-sm text-fore-30 font-semibold pl-14'>{group.name}</div>
										{group.items.map((item) => {
											return (
												<Item
													onSelected={() => {
														if (!item.event) return

														setItem(item.event)
													}}
													isSelected={item.event ? selectedKey === getEventId(item.event) : false}
													{...item}
												/>
											)
										})}
									</div>
								</>
							)
						})}

						{props.footer &&
							(
								<div class='flex-1 flex flex-col'>
									<div class='flex-1' />
									<div>
										<ComponentRender {...props.footer} />
									</div>
								</div>
							)}
					</div>
				</div>
			</div>
			<div class='w-2 bg-fore-10' />
			<div class='flex-1 bg-gradient-to-bl from-secondary-15 via-primary-1 to-transparent'>
				{props.body && <ComponentRender {...props.body} />}
			</div>
		</div>
	)
}

function Item(props: SidebarItem & { isSelected: boolean; onSelected(): void }) {
	const { isLoading, dispatch, isDisabled } = useDispatcher(props.event ?? null)

	// TODO implement isLoading

	const selectionStyle = props.isSelected
		? 'bg-fore-5 text-fore-80 shadow-sm backdrop-blur-md'
		: 'bg-transparent hover:bg-fore-2 text-fore-40'

	return (
		<button
			disabled={isDisabled}
			class={`flex gap-10 items-center rounded py-8 px-14 font-semibold transition-colors ${selectionStyle}`}
			onClick={() => dispatch(null).then(() => props.onSelected())}
		>
			{props.icon && (
				<IconRender
					name={props.icon}
					color={{ kind: props.isSelected ? 'Primary' : 'Fore', opacity: props.isSelected ? 100 : 40 }}
					size={20}
				/>
			)}
			<div>{props.title}</div>
		</button>
	)
}
