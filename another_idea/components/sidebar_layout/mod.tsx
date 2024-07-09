import { isOk, React, SidebarItem, SidebarLayout, useDispatcher } from 'runtime'
import { ComponentRender } from '../mod.tsx'
import { ImageRender } from '../image/mod.tsx'
import { IconRender } from '../icon/mod.tsx'

export function SidebarLayoutRender(props: SidebarLayout) {
	const [selectedKey, setSelectedKey] = React.useState(props.initial_action ?? null)

	const setItem = (key: string) => setSelectedKey(key)

	return (
		<div class='h-full flex'>
			<div class='w-300 bg-fore-5'>
				<div class='h-full p-20 flex flex-col gap-40 bg-gradient-to-tr from-secondary-15 via-primary-1 to-transparent backdrop-blur-md'>
					<button
						class='flex gap-10 items-center'
						onClick={() => {
							if (!isOk(props.title_action)) return

							setItem(props.title_action)
						}}
					>
						{props.logo && <ImageRender {...props.logo} />}
						<div class='text-xl font-bold'>{props.title}</div>
					</button>

					{props.action_items.length > 0 && (
						<div class='flex flex-col gap-10'>
							{props.action_items.map((item) => (
								<Item
									onSelected={() => {
										if (!isOk(item.action)) return

										setItem(item.action)
									}}
									isSelected={isOk(selectedKey) ? selectedKey === item.action : false}
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
														if (!isOk(item.action)) return

														setItem(item.action)
													}}
													isSelected={isOk(selectedKey) ? selectedKey === item.action : false}
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
	const { isLoading, dispatch, isDisabled } = useDispatcher(props.action ?? null)

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
					color={{ type: props.isSelected ? 'Primary' : 'Fore', opacity: props.isSelected ? 100 : 40 }}
					size={20}
				/>
			)}
			<div>{props.title}</div>
		</button>
	)
}
