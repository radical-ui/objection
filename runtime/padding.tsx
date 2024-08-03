import { ComponentRender } from './component.tsx'
import { Component, React } from './deps.ts'

/**
 * A container with padding.
 *
 * **Example**
 *
 * ```rust Padding::all(30).body(Card::new().body(Label::new("See, it is padded!"))) ```
 *
 * @component
 */
export interface Padding {
	body?: Component
	bottom?: number
	left?: number
	right?: number
	top?: number
	all?: number
	horizontal?: number
	vertical?: number
}

export function PaddingRender(props: Padding) {
	const top = props.top ?? props.vertical ?? props.all ?? 0
	const bottom = props.bottom ?? props.vertical ?? props.all ?? 0
	const right = props.right ?? props.horizontal ?? props.all ?? 0
	const left = props.left ?? props.horizontal ?? props.all ?? 0

	return (
		<div class={`w-full h-full pt-${top} pl-${left} pr-${right} pb-${bottom}`}>
			{props.body ? <ComponentRender {...props.body} /> : <></>}
		</div>
	)
}
