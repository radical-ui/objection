<script>
	import { onMount } from 'svelte';
	import { numberOfCreated } from '../store.js';
	import rawStyle from '../styles/internal/wrapped-styles.js';

	let create = false;
	let style = null;
	let id = `svelte-toolbox-style-id-${(Math.random() * Math.random() * new Date().getTime()).toFixed(0)}`;

	numberOfCreated.update(num => {num.push(id); return num;})

	onMount(async () => {
		if (id == $numberOfCreated[0]) {
			style = `<style>${rawStyle}</style>`;
			create = true
		}
	})
</script>

<svelte:head>
	{#if create}
		{@html style}
	{/if}
</svelte:head>
