<script module>
	export const hello = flex(30).direction('column')
</script>

<script lang="ts">
	import ElementIndex from '~/element_index.svelte'
	import { type Flex, flex } from '~/element_type'

	const { direction, gap = 0, justify = 'flex-start', align = 'stretch', children = [] }: Flex = $props()
</script>

<div
	style:display="flex"
	style="
		flex-direction: {direction};
		gap: {gap};
		justify-content: {justify};
		align-items: {align};
		width: 100%;
		height: 100%;
	"
>
	{#each children as child}
		<div
			style:flex={child.expand ? '1' : null}
			style:min-height={direction === 'column' && child.expand ? '0' : null}
			style:min-width={direction === 'row' && child.expand ? '0' : null}
		>
			<ElementIndex {...child} />
		</div>
	{/each}
</div>
