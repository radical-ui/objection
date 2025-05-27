import { mount } from 'svelte'
import { getSyncUrl } from '../bridge'
import Root from '../root.svelte'
import type { Element } from '~/element_type'

const emptyState: Element = {
	$: 'flex',
	children: [],
	align: 'center',
	direction: 'column',
	gap: 0,
	justify: 'center',
}

mount(Root, {
	target: document.body,
	props: {
		syncUrl: getSyncUrl(),
		state: emptyState,
	},
})
