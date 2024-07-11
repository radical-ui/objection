import { Header, HeaderSize, React } from 'runtime'
import { LabelRender } from '../label/mod.tsx'
import { IconButtonRender } from '../icon_button/mod.tsx'

const getIconSize = (size: HeaderSize) => {
	if (size === 'Large') return 25
	if (size === 'Medium') return 20
	if (size === 'Small') return 20

	throw new Error('unknown size')
}

const getTextSize = (size: HeaderSize) => {
	if (size === 'Large') return '2xl'
	if (size === 'Medium') return 'xl'
	if (size === 'Small') return 'lg'
}

export function SimpleLayoutRender(props: Header) {
	return (
		<div class='flex-1 flex flex-col gap-5'>
			<div class='flex gap-5 items-center'>
				<h1 class={`flex-1 text-primary flex text-${getTextSize(props.size)}`}>
					<LabelRender
						color={{ type: 'Fore', opacity: 90 }}
						is_bold
						is_italic={false}
						text={props.title}
						edit_action={props.title_edit_action}
						placeholder={props.title_placeholder}
					/>
				</h1>
				{props.action_items.map((item) => (
					<div class='h-0 flex items-center'>
						<IconButtonRender
							color={{ type: 'Primary', opacity: 100 }}
							name={item.icon}
							size={getIconSize(props.size)}
							action={item.action}
							title={item.label}
						/>
					</div>
				))}
			</div>
			{props.subtitle && (
				<h3 class='text-fore-30 flex'>
					<LabelRender
						text={props.subtitle}
						is_bold={false}
						is_italic={false}
						color={{ type: 'Fore', opacity: 30 }}
						edit_action={props.subtitle_edit_action}
						placeholder={props.subtitle_placeholder}
					/>
				</h3>
			)}
		</div>
	)
}
