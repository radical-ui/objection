<script>
	import Link from './Link.svelte';
	import { fade, scale } from 'svelte/transition';

	export let name = 'error';
	export let href = null;
	export let style = '';
	export let color = 'var(--icon-buttons)';
	export let newTab = false;

	let cs;
	$: cs = `color: ${color}; ${style};`;
</script>

<style>
	i {
		margin: 10px;
	}
</style>

<Link {href} {newTab}>
	<i class="material-icons" style={cs}>{name}</i>
</Link>
