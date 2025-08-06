<script lang="ts">
	import ElementIndex from './index.svelte'
	import { type Flex } from '~/element_type'
	import { getCSSContext } from '~/theme'

	const { direction, gap = 0, justify = 'flex-start', align = 'stretch', children = [] }: Flex = $props()

	// Get CSS context
	const { sizeToCSS } = getCSSContext()
</script>

<div
	style:display="flex"
	style="
		flex-direction: {direction};
		gap: {sizeToCSS(gap) || '0'};
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
