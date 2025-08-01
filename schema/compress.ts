import type { ElementInfo } from './def'

export const compress = {
	description: 'An element that removes all concrete contraints along both axis.',
	params: {
		preserveX: {
			type: 'boolean',
			description: 'If true, constraints will be preserved along the X axis.',
		},
		preserveY: {
			type: 'boolean',
			description: 'If true, constraints will be preserved along the Y axis.',
		},
		child: {
			type: 'element',
			description: 'The child element.',
			required: true,
		},
	},
} satisfies ElementInfo
