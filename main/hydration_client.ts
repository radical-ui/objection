import { hydrate } from 'svelte'
import { getHydrationState, getSyncUrl } from '../bridge'
import Root from '../root.svelte'

hydrate(Root, {
	target: document.body,
	props: {
		syncUrl: getSyncUrl(),
		state: getHydrationState(),
	},
})
