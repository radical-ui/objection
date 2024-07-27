import { useDispatcher } from './event.tsx'
import { Component, ComponentRender } from './component.tsx'
import { EventKey, React } from './deps.ts'
import { IconRender } from './icon.tsx'
import { LabelRender } from './label.tsx'

/**
 * TODO
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Batter, }
 *
 * Flex::new(FlexKind::Column) .gap(30) .auto_item( RadioInput::new() .event(Event::Batter) .item(0, "Red") .item(1, "Green") ) .auto_item( RadioInput::new() .event(Event::Batter) .item(0, "Hi") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) .auto_item( RadioInput::new() .item(0, "all are disabled here") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) ```
 *
 * @component
 */
export interface RadioInput {
	event?: EventKey<number>
	initialValue?: number
	items: RadioItem[]
}
export interface RadioItem {
	description?: Component
	id: number
	title: string
}

export function RadioInputRender(props: RadioInput) {
	const { dispatch, isDisabled } = useDispatcher(props.event ?? null)
	const [selectedId, setSelectedId] = React.useState<number | null>(props.initialValue ?? null)

	const someoneHasDescription = !!props.items.find((item) => item.description)
	const goVertical = someoneHasDescription || props.items.length > 4

	return (
		<div class={`flex gap-${goVertical ? 10 : 20} ${goVertical ? 'flex-col' : ''}`}>
			{props.items.map((item) => {
				const isSelected = item.id === selectedId

				return (
					<button
						class={`
							text-left flex flex-col transition-opacity 
							${isDisabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:opacity-90'}
						`}
						disabled={isDisabled}
						onClick={() => {
							if (props.event) dispatch(item.id).then(() => setSelectedId(item.id))
						}}
					>
						<div class='flex items-center gap-5'>
							<IconRender
								name={isSelected ? 'mdi-radiobox-marked' : 'mdi-radiobox-blank'}
								size={20}
								color={isSelected ? { opacity: 100, kind: 'Primary' } : { opacity: 50, kind: 'Fore' }}
							/>
							<LabelRender color={{ kind: 'Fore', opacity: 50 }} isBold isItalic={false} text={item.title} />
						</div>
						{item.description && (
							<div class='text-fore-40 pl-25'>
								<ComponentRender {...item.description} />
							</div>
						)}
					</button>
				)
			})}
		</div>
	)
}
