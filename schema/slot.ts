import type { ElementInfo } from './def'

export const slot = {
	description: 'An ephemeral container for replaceable content',
	params: {
		slot_id: {
			description: 'The unique identifier for this slot.',
			type: 'text',
			required: true,
		},
		content: {
			description: 'The element inside the slot. This content can be replaced using the slot id and the `slot` downstream message.',
			type: 'element',
		},
	},
} satisfies ElementInfo
