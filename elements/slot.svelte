<script lang="ts">
	import { type Slot } from '~/element_type'
	import ElementIndex from './index.svelte'

	const { content }: Slot = $props()
</script>

{#if content}
	<ElementIndex {...content} />
{/if}
