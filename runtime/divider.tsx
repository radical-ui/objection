import { React } from './deps.ts'

const getBreadth = (distinction: DividerDistinction) => {
	if (distinction === 'Profound') return 4
	if (distinction === 'Medium') return 2
	if (distinction === 'Slight') return 1
}

export type DividerDirection = 'Horizontal' | 'Vertical'
export type DividerDistinction = 'Profound' | 'Medium' | 'Slight'

/**
 * A visual divider, which can be horizontal or vertical, and can have varying distinction.
 *
 * **Example**
 *
 * ```rust Flex::new(FlexKind::Column) .gap(10) .auto_item("Slight") .auto_item(Divider::new().distinction(DividerDistinction::Slight)) .auto_item("Medium") .auto_item(Divider::new().distinction(DividerDistinction::Medium)) .auto_item("Profound") .auto_item(Divider::new().distinction(DividerDistinction::Profound)) ```
 *
 * @component
 */
export interface Divider {
	direction: DividerDirection
	distinction: DividerDistinction
}

export function DividerRender(props: Divider) {
	const mainLetter = props.direction === 'Horizontal' ? 'w' : 'h'
	const crossLetter = props.direction === 'Vertical' ? 'w' : 'h'

	return <div class={`${mainLetter}-full ${crossLetter}-${getBreadth(props.distinction)} bg-fore-10`} />
}
