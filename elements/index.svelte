<script lang="ts">
	import type { Element } from '~/element_type'
	import Container from './container.svelte'
	import Compress from './compress.svelte'
	import Flex from './flex.svelte'
	import Slot from './slot.svelte'

	const element: Element = $props()
</script>

{#if element.$ === 'container'}
	<Container {...element} />
{:else if element.$ === 'compress'}
	<Compress {...element} />
{:else if element.$ === 'flex'}
	<Flex {...element} />
{:else if element.$ === 'slot'}
	<Slot {...element} />
{/if}
