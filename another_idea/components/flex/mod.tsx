import { Flex, FlexAlign, FlexJustify, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function FlexRender(params: Flex) {
	return (
		<div
			class={`w-full h-full flex ${params.kind === 'Column' ? 'flex-col' : ''} gap-${params.gap}`}
			style={{ alignItems: alignment(params.align), justifyContent: justify(params.justify) }}
		>
			{params.items.map((item) => {
				return (
					<div class={`${item.growth === 'Expand' ? 'flex-1' : ''} min-${params.kind === 'Column' ? 'h' : 'w'}-0`}>
						<ComponentRender {...item.component} />
					</div>
				)
			})}
		</div>
	)
}

const alignment = (input: FlexAlign) => {
	if (input === 'Center') return 'center'
	if (input === 'Baseline') return 'baseline'
	if (input === 'End') return 'end'
	if (input === 'Start') return 'start'
	if (input === 'SafeCenter') return 'safe center'
	if (input === 'Stretch') return 'stretch'

	console.warn(`Unknown flex alignment "${input}", resorting to "stretch"`)
	return 'stretch'
}

const justify = (input: FlexJustify) => {
	if (input === 'Center') return 'center'
	if (input === 'End') return 'end'
	if (input === 'Start') return 'start'
	if (input === 'SafeCenter') return 'safe center'
	if (input === 'SpaceAround') return 'space-around'
	if (input === 'SpaceBetween') return 'space-between'
	if (input === 'SpaceEvenly') return 'space-evenly'
	if (input === 'Stretch') return 'stretch'

	console.warn(`Unknown flex justify "${input}", resorting to "start"`)
	return 'start'
}
