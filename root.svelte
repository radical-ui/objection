<script lang="ts">
	import ElementIndex from './elements/index.svelte'
	import type { Element } from './element_type'
	import { createCSSContext, setCSSContext } from './theme'

	const props: { syncUrl: URL | undefined; state: Element } = $props()

	// Set up CSS context with default mappings
	const cssContext = createCSSContext()
	setCSSContext(cssContext)
</script>

<ElementIndex {...props.state} />

<style>
	:global(html, body) {
		width: 100%;
		height: 100%;
		overflow: hidden;
	}
</style>
