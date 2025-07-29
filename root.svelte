<script lang="ts">
	import ElementIndex from './element_index.svelte'
	import type { Element } from './element_type'

	const props: { syncUrl: URL | undefined; state: Element } = $props()
</script>

<ElementIndex {...props.state} />
