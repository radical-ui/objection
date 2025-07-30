import { getChildElements, type Element } from '~/element_type'

// Mutates the base in place, finding `slotId` and setting the content to `slot`
export function reconcileSlot(base: Element, slotId: string, slot: Element) {
	for (const child of getChildElements(base)) {
		if (child.$ === 'slot' && child.slot_id === slotId) {
			child.content = slot
			return
		}

		reconcileSlot(child, slotId, slot)
	}
}
