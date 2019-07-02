<script>
	import Link from './Link.svelte';
	import { fade, scale } from 'svelte/transition';

	export let name = 'error';
	export let href = null;
	export let style = '';
	export let color = 'var(--icon-buttons)';
	export let newTab = false;
	export let transition = null; // `scale`, `fade`, `swipe`, or `zoom`

	let cs;
	$: cs = `color: ${color}; ${style};`;
</script>

<style>
	i {
		margin: 10px;
	}
</style>

<Link {href} {newTab}>
	{#if transition == `scale`}
		<i class="material-icons" style={cs} in:scale={{ delay: 500 }} out:fade={{ duration: 400 }}>{name}</i>
	{:else if transition == `fade`}
		<i class="material-icons" style={cs}>{name}</i>
	{:else if transition == `swipe`}
		<i class="material-icons" style={cs}>{name}</i>
	{:else if transition == `zoom`}
		<i class="material-icons" style={cs}>{name}</i>
	{:else}
		<i class="material-icons" style={cs}>{name}</i>
	{/if}
</Link>