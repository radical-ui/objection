// This file is auto-generated. Do not edit directly.

/** Accepts pixels (number) or preset sizes */
export type Size = number | 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '4xl' | 'full'

export type RgbaColor = [number, number, number, number]
export type StaticColor = 'primary' | 'success' | 'danger' | 'warning' | 'info' | 'foreground' | 'background' | 'border' | 'focus' | 'muted' | 'muted_foreground'
/** Accepts RGBA values (array of 4 numbers) or a static color */
export type Color = RgbaColor | StaticColor

export type Element = Flex | Slot | Container | Compress

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

/** A basic container element. By default, it will fill all available space, but if `compress` is supplied, it will shrink to the size of its child. */
export type Container = {
	$: 'container'

	/** The child element to be contained within the container. If the child is larger than the container, it will be placed above sibling elements, keeping their layout unaffected. */
	child?: Element

	/** The background color of the container. If unsupplied, it will be transparent. */
	color?: Color

	/** The padding inside the container. */
	padding?: Size

	/** The horizontal padding inside the container. Overrides `padding`. */
	paddingX?: Size

	/** The vertical padding inside the container. Overrides `padding`. */
	paddingY?: Size

	/** The top padding inside the container. Overrides `padding` and `paddingY`. */
	paddingTop?: Size

	/** The bottom padding inside the container. Overrides `padding` and `paddingY`. */
	paddingBottom?: Size

	/** The left padding inside the container. Overrides `padding` and `paddingX`. */
	paddingLeft?: Size

	/** The right padding inside the container. Overrides `padding` and `paddingX`. */
	paddingRight?: Size

	/** The border radius of the container. Does not create a border; use `border` for that. If a background color is supplied, it will be clipped. Specify `clip` if you want the child to be clipped also. */
	borderRadius?: Size

	/** The color of the border. Does not create a border; use `border` for that. If unsupplied, the border will be colored to the `border` color. */
	borderColor?: Color

	/** The size of the border. If supplied, a border is created. No other param will create the border, they just configure the border created by this param. */
	border?: Size

	/** If true, the child will be clipped so that it does not grow outside of this container. */
	clip?: boolean

	/** The position of the child inside this container, defaulting to `center`. Works when the child is larger than the container and when the child is smaller than the container. If they are the exact same size, the effect is not percieved. */
	childPosition?: 'top_left' | 'top_center' | 'top_right' | 'center_left' | 'center' | 'center_left' | 'bottom_left' | 'bottom_center' | 'bottom_right'

	/** The width and height of the container. Naturally, this gives the child a concrete constraint to expand against. */
	size?: Size

	/** The width of the container, overriding `size`. Naturally, this gives the child a concrete constraint to expand against along the X axis. */
	width?: Size

	/** The height of the container, overriding `size`. Naturally, this gives the child a concrete constraint to expand against along the Y axis. */
	height?: Size
}

/** An element that removes all concrete contraints along both axis. */
export type Compress = {
	$: 'compress'

	/** If true, constraints will be preserved along the X axis. */
	preserveX?: boolean

	/** If true, constraints will be preserved along the Y axis. */
	preserveY?: boolean

	/** The child element. */
	child: Element
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

/** A basic container element. By default, it will fill all available space, but if `compress` is supplied, it will shrink to the size of its child. */
export class ContainerBuilder {
	state: Container

	constructor(state: Container) {
		this.state = state
	}

	/** The child element to be contained within the container. If the child is larger than the container, it will be placed above sibling elements, keeping their layout unaffected. */
	child(child: Element) {
		this.state.child = child
		return this
	}

	/** The background color of the container. If unsupplied, it will be transparent. */
	color(color: Color) {
		this.state.color = color
		return this
	}

	/** The padding inside the container. */
	padding(padding: Size) {
		this.state.padding = padding
		return this
	}

	/** The horizontal padding inside the container. Overrides `padding`. */
	paddingX(paddingX: Size) {
		this.state.paddingX = paddingX
		return this
	}

	/** The vertical padding inside the container. Overrides `padding`. */
	paddingY(paddingY: Size) {
		this.state.paddingY = paddingY
		return this
	}

	/** The top padding inside the container. Overrides `padding` and `paddingY`. */
	paddingTop(paddingTop: Size) {
		this.state.paddingTop = paddingTop
		return this
	}

	/** The bottom padding inside the container. Overrides `padding` and `paddingY`. */
	paddingBottom(paddingBottom: Size) {
		this.state.paddingBottom = paddingBottom
		return this
	}

	/** The left padding inside the container. Overrides `padding` and `paddingX`. */
	paddingLeft(paddingLeft: Size) {
		this.state.paddingLeft = paddingLeft
		return this
	}

	/** The right padding inside the container. Overrides `padding` and `paddingX`. */
	paddingRight(paddingRight: Size) {
		this.state.paddingRight = paddingRight
		return this
	}

	/** The border radius of the container. Does not create a border; use `border` for that. If a background color is supplied, it will be clipped. Specify `clip` if you want the child to be clipped also. */
	borderRadius(borderRadius: Size) {
		this.state.borderRadius = borderRadius
		return this
	}

	/** The color of the border. Does not create a border; use `border` for that. If unsupplied, the border will be colored to the `border` color. */
	borderColor(borderColor: Color) {
		this.state.borderColor = borderColor
		return this
	}

	/** The size of the border. If supplied, a border is created. No other param will create the border, they just configure the border created by this param. */
	border(border: Size) {
		this.state.border = border
		return this
	}

	/** If true, the child will be clipped so that it does not grow outside of this container. */
	clip(clip: boolean) {
		this.state.clip = clip
		return this
	}

	/** The position of the child inside this container, defaulting to `center`. Works when the child is larger than the container and when the child is smaller than the container. If they are the exact same size, the effect is not percieved.
 * `top_left` | `top_center` | `top_right` | `center_left` | `center` | `center_left` | `bottom_left` | `bottom_center` | `bottom_right` */
	childPosition(childPosition: 'top_left' | 'top_center' | 'top_right' | 'center_left' | 'center' | 'center_left' | 'bottom_left' | 'bottom_center' | 'bottom_right') {
		this.state.childPosition = childPosition
		return this
	}

	/** The width and height of the container. Naturally, this gives the child a concrete constraint to expand against. */
	size(size: Size) {
		this.state.size = size
		return this
	}

	/** The width of the container, overriding `size`. Naturally, this gives the child a concrete constraint to expand against along the X axis. */
	width(width: Size) {
		this.state.width = width
		return this
	}

	/** The height of the container, overriding `size`. Naturally, this gives the child a concrete constraint to expand against along the Y axis. */
	height(height: Size) {
		this.state.height = height
		return this
	}
}

/** An element that removes all concrete contraints along both axis. */
export class CompressBuilder {
	state: Compress

	constructor(state: Compress) {
		this.state = state
	}

	/** If true, constraints will be preserved along the X axis. */
	preserveX(preserveX: boolean) {
		this.state.preserveX = preserveX
		return this
	}

	/** If true, constraints will be preserved along the Y axis. */
	preserveY(preserveY: boolean) {
		this.state.preserveY = preserveY
		return this
	}

	/** The child element. */
	child(child: Element) {
		this.state.child = child
		return this
	}
}

export function flex() {
	return new FlexBuilder({ $: 'flex' })
}

export function slot(slot_id: string) {
	return new SlotBuilder({ $: 'slot', slot_id })
}

export function container() {
	return new ContainerBuilder({ $: 'container' })
}

export function compress(child: Element) {
	return new CompressBuilder({ $: 'compress', child })
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
		case 'container':
			if (element.child) children.push(element.child)
			break
		case 'compress':
			if (element.child) children.push(element.child)
			break
	}
	
	return children
}
