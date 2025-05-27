<script lang="ts">
	import ElementIndex from '~/element_index.svelte'
	import type { Flex } from '~/element_type'

	const { direction, gap = 0, justify = 'flex-start', align = 'stretch', children = [] }: Flex = $props()
</script>

<div
	style:display="flex"
	style="
		flex-direction: {direction};
		gap: {gap};
		justify-content: {justify};
		align-items: {align};
	"
>
	{#each children as child}
		<div style:flex={child.expand ? '1' : null} style="">
			<ElementIndex {...child} />
		</div>
	{/each}
</div>
