<script lang="ts">
	import ElementIndex from './elements/index.svelte'
	import type { Element } from './element_type'

	const props: { syncUrl: URL | undefined; state: Element } = $props()
</script>

<ElementIndex {...props.state} />

<style>
	:global(html, body) {
		width: 100%;
		height: 100%;
		overflow: hidden;
	}
</style>
