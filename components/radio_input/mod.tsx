import { isOk, RadioInput, React, useDispatcher } from 'runtime'
import { IconRender } from '../icon/mod.tsx'
import { ComponentRender } from '../mod.tsx'
import { LabelRender } from '../label/mod.tsx'

export function RadioInputRender(props: RadioInput) {
	const { dispatch, isDisabled } = useDispatcher(props.action ?? null)
	const [selectedId, setSelectedId] = React.useState<number | null>(props.initial_value ?? null)

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
							if (isOk(props.action)) dispatch(item.id).then(() => setSelectedId(item.id))
						}}
					>
						<div class='flex items-center gap-5'>
							<IconRender
								name={isSelected ? 'mdi-radiobox-marked' : 'mdi-radiobox-blank'}
								size={20}
								color={isSelected ? { opacity: 100, type: 'Primary' } : { opacity: 50, type: 'Fore' }}
							/>
							<LabelRender color={{ type: 'Fore', opacity: 50 }} is_bold is_italic={false} text={item.title} />
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
