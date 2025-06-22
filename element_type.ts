// This file is auto-generated. Do not edit directly.

/** Accepts pixels (number) or preset sizes */
export type Size = number | 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl' | 'full'

export type Element = Flex

/** A flexible layout container */
export type Flex = {
	$: 'flex'

	/** The direction of the flex layout. */
	direction?: 'row' | 'column' | 'row-reverse' | 'column-reverse'

	/** Spacing between children. */
	gap?: Size

	/** Horizontal alignment of children within the container. */
	justify?: 'flex-start' | 'center' | 'flex-end' | 'space-between' | 'space-around' | 'space-evenly'

	/** Vertical alignment of children within the container. */
	align?: 'stretch' | 'flex-start' | 'center' | 'flex-end' | 'baseline'

	/** Child elements inside the flex container. */
	children?: (Element & {
		/** Whether the element should expand to fill available space */
		expand: boolean
		/** The flex-grow factor determining how much available space the element should take up relative to other expanding elements */
		expansionBasis: number
	})[]
}

/** A flexible layout container */
export class FlexBuilder {
	state: Flex

	constructor(state: Flex) {
		this.state = state
	}

	/** The direction of the flex layout.
	 * `row` | `column` | `row-reverse` | `column-reverse` */
	direction(direction: 'row' | 'column' | 'row-reverse' | 'column-reverse') {
		this.state.direction = direction
		return this
	}

	/** Spacing between children. */
	gap(gap: Size) {
		this.state.gap = gap
		return this
	}

	/** Horizontal alignment of children within the container.
	 * `flex-start` | `center` | `flex-end` | `space-between` | `space-around` | `space-evenly` */
	justify(justify: 'flex-start' | 'center' | 'flex-end' | 'space-between' | 'space-around' | 'space-evenly') {
		this.state.justify = justify
		return this
	}

	/** Vertical alignment of children within the container.
	 * `stretch` | `flex-start` | `center` | `flex-end` | `baseline` */
	align(align: 'stretch' | 'flex-start' | 'center' | 'flex-end' | 'baseline') {
		this.state.align = align
		return this
	}

	/** Child elements inside the flex container. */
	children(
		children: (Element & {
			/** Whether the element should expand to fill available space */
			expand: boolean
			/** The flex-grow factor determining how much available space the element should take up relative to other expanding elements */
			expansionBasis: number
		})[]
	) {
		this.state.children = children
		return this
	}
}

export function flex(gap: number) {
	return new FlexBuilder({ $: 'flex', gap })
}
