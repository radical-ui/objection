// This file is auto-generated. Do not edit directly.

/** Accepts pixels (number) or preset sizes */
export type Size = number | 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl' | 'full'

export type Element = Flex | Slot

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

/** An ephemeral container for replaceable content */
export type Slot = {
	$: 'slot'

	/** The unique identifier for this slot. */
	slot_id: string

	/** The element inside the slot. This content can be replaced using the slot id and the `slot` downstream message. */
	content?: Element
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
	children(children: (Element & {
		/** Whether the element should expand to fill available space */
		expand: boolean
		/** The flex-grow factor determining how much available space the element should take up relative to other expanding elements */
		expansionBasis: number
	})[]) {
		this.state.children = children
		return this
	}
}

/** An ephemeral container for replaceable content */
export class SlotBuilder {
	state: Slot

	constructor(state: Slot) {
		this.state = state
	}

	/** The unique identifier for this slot. */
	slot_id(slot_id: string) {
		this.state.slot_id = slot_id
		return this
	}

	/** The element inside the slot. This content can be replaced using the slot id and the `slot` downstream message. */
	content(content: Element) {
		this.state.content = content
		return this
	}
}

export function flex() {
	return new FlexBuilder({ $: 'flex' })
}

export function slot(slot_id: string) {
	return new SlotBuilder({ $: 'slot', slot_id })
}

/** Gets all child elements for a given element */
export function getChildElements(element: Element): Element[] {
	const children: Element[] = []
	
	switch (element.$) {
		case 'flex':
			if (element.children) children.push(...element.children)
			break
		case 'slot':
			if (element.content) children.push(element.content)
			break
	}
	
	return children
}
