import { ComponentRender } from './component.tsx'
import { Component, React } from './deps.ts'

export type FlexKind = 'Row' | 'Column'
export type FlexAlign = 'Stretch' | 'Center' | 'Start' | 'End' | 'Baseline' | 'SafeCenter'
export type FlexGrowth = 'Auto' | 'Expand'
export type FlexJustify =
	| 'Center'
	| 'SafeCenter'
	| 'Start'
	| 'End'
	| 'SpaceBetween'
	| 'SpaceAround'
	| 'SpaceEvenly'
	| 'Stretch'

/**
 * @component
 */
export interface Flex {
	align?: FlexAlign
	gap?: number
	items?: FlexItem[]
	justify?: FlexJustify
	kind: FlexKind
}

export interface FlexItem {
	growth: FlexGrowth
	component: Component
}

export function FlexRender(params: Flex) {
	const align = params.align || 'Stretch'
	const justify = params.justify || 'Start'
	const items = params.items || []

	return (
		<div
			class={`w-full h-full flex ${params.kind === 'Column' ? 'flex-col' : ''} gap-${params.gap}`}
			style={{ alignItems: alignment(align), justifyContent: justification(justify) }}
		>
			{items.map((item) => {
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

const justification = (input: FlexJustify) => {
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
