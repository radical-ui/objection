<script lang="ts">
	import type { Component } from 'svelte'
	import type { Element } from './element_type'

	// @ts-expect-error this is from the plugin defined in `cli/plugins/element_picker_plugin.ts`
	import pickElement from '@internal/element_picker'

	const element: Element = $props()

	const CustomComponent = pickElement(element.$) as Component<Element>
</script>

<CustomComponent {...element} />

<style>
	:global(html, body) {
		width: 100%;
		height: 100%;
		overflow: hidden;
	}
</style>
