import { Component, ComponentRender } from './component.tsx'
import { React } from './deps.ts'

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
	bottom: number
	left: number
	right: number
	top: number
}

export function PaddingRender(props: Padding) {
	return (
		<div class={`w-full h-full pt-${props.top} pl-${props.left} pr-${props.right} pb-${props.bottom}`}>
			{props.body ? <ComponentRender {...props.body} /> : <></>}
		</div>
	)
}
