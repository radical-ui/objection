import type { ElementInfo } from './def'

export const container = {
	description:
		'A basic container element. By default, it will fill all available space, but if `compress` is supplied, it will shrink to the size of its child.',
	params: {
		child: {
			type: 'element',
			description:
				'The child element to be contained within the container. If the child is larger than the container, it will be placed above sibling elements, keeping their layout unaffected.',
		},
		color: {
			type: 'color',
			description: 'The background color of the container. If unsupplied, it will be transparent.',
		},
		padding: {
			type: 'size',
			description: 'The padding inside the container.',
		},
		paddingX: {
			type: 'size',
			description: 'The horizontal padding inside the container. Overrides `padding`.',
		},
		paddingY: {
			type: 'size',
			description: 'The vertical padding inside the container. Overrides `padding`.',
		},
		paddingTop: {
			type: 'size',
			description: 'The top padding inside the container. Overrides `padding` and `paddingY`.',
		},
		paddingBottom: {
			type: 'size',
			description: 'The bottom padding inside the container. Overrides `padding` and `paddingY`.',
		},
		paddingLeft: {
			type: 'size',
			description: 'The left padding inside the container. Overrides `padding` and `paddingX`.',
		},
		paddingRight: {
			type: 'size',
			description: 'The right padding inside the container. Overrides `padding` and `paddingX`.',
		},
		borderRadius: {
			type: 'size',
			description:
				'The border radius of the container. Does not create a border; use `border` for that. If a background color is supplied, it will be clipped. Specify `clip` if you want the child to be clipped also.',
		},
		borderColor: {
			type: 'color',
			description:
				'The color of the border. Does not create a border; use `border` for that. If unsupplied, the border will be colored to the `border` color.',
		},
		border: {
			type: 'size',
			description:
				'The size of the border. If supplied, a border is created. No other param will create the border, they just configure the border created by this param.',
		},
		clip: {
			type: 'boolean',
			description: 'If true, the child will be clipped so that it does not grow outside of this container.',
		},
		childPosition: {
			type: 'enum',
			description:
				'The position of the child inside this container, defaulting to `center`. Works when the child is larger than the container and when the child is smaller than the container. If they are the exact same size, the effect is not percieved.',
			options: [
				{ id: 'top_left', description: 'Place the child in the top-left corner.' },
				{ id: 'top_center', description: 'Place the child in the center of the top.' },
				{ id: 'top_right', description: 'Place the child in the top-right corner.' },
				{ id: 'center_left', description: 'Place the child in the center of the left side of this container.' },
				{ id: 'center', description: 'Place the child in the center of this container.' },
				{ id: 'center_left', description: 'Place the child in the center of the right side of this container.' },
				{ id: 'bottom_left', description: 'Place the child in the bottom-left corner.' },
				{ id: 'bottom_center', description: 'Place the child in the center of the bottom.' },
				{ id: 'bottom_right', description: 'Place the child in the bottom-right corner.' },
			],
		},
		size: {
			type: 'size',
			description: 'The width and height of the container. Naturally, this gives the child a concrete constraint to expand against.',
		},
		width: {
			type: 'size',
			description:
				'The width of the container, overriding `size`. Naturally, this gives the child a concrete constraint to expand against along the X axis.',
		},
		height: {
			type: 'size',
			description:
				'The height of the container, overriding `size`. Naturally, this gives the child a concrete constraint to expand against along the Y axis.',
		},
	},
} satisfies ElementInfo
