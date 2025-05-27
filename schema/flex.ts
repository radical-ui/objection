import type { ElementInfo } from './def'

export const flex = {
	description: 'A flexible layout container',
	params: {
		direction: {
			description: 'The direction of the flex layout.',
			type: 'enum',
			options: [
				{ id: 'row', description: 'Left to right' },
				{ id: 'column', description: 'Top to bottom' },
				{ id: 'row-reverse', description: 'Right to left' },
				{ id: 'column-reverse', description: 'Bottom to top' },
			],
		},
		gap: {
			description: 'Spacing between children.',
			type: 'size',
		},
		justify: {
			description: 'Horizontal alignment of children within the container.',
			type: 'enum',
			options: [
				{ id: 'flex-start', description: 'Align items to the start' },
				{ id: 'center', description: 'Align items to the center' },
				{ id: 'flex-end', description: 'Align items to the end' },
				{ id: 'space-between', description: 'Even spacing between items' },
				{ id: 'space-around', description: 'Even spacing around items' },
				{ id: 'space-evenly', description: 'Equal spacing' },
			],
		},
		align: {
			description: 'Vertical alignment of children within the container.',
			type: 'enum',
			options: [
				{ id: 'stretch', description: 'Stretch to fill container' },
				{ id: 'flex-start', description: 'Align to start' },
				{ id: 'center', description: 'Align to center' },
				{ id: 'flex-end', description: 'Align to end' },
				{ id: 'baseline', description: 'Align to text baseline' },
			],
		},
		children: {
			description: 'Child elements inside the flex container.',
			type: 'element_list',
			extend_params: {
				expand: {
					type: 'boolean',
					description: 'Whether the element should expand to fill available space',
				},
				expansionBasis: {
					type: 'number',
					description:
						'The flex-grow factor determining how much available space the element should take up relative to other expanding elements',
				},
			},
		},
	},
} satisfies ElementInfo
