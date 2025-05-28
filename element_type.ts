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
