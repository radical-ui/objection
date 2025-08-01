<script lang="ts">
	import { type Compress } from '~/element_type'
	import ElementIndex from './index.svelte'

	const { child, preserveX, preserveY }: Compress = $props()
</script>

<div style:width={preserveX ? '100%' : undefined} style:height={preserveY ? '100%' : undefined}>
	<ElementIndex {...child} />
</div>
