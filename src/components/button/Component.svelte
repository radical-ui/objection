<style>
	button {
		font-size: 1.2rem;
		padding: 0 12px;
		line-height: 2rem;
		font-weight: 500;
		font-family: Roboto, Helvetica, Arial, sans-serif;
		box-shadow: 0 2px 2px 0 rgba(0, 0, 0, 0.14),
			0 3px 1px -2px rgba(0, 0, 0, 0.2), 0 1px 5px 0 rgba(0, 0, 0, 0.12);
		border: none;
		background: rgba(0, 0, 0, 0);
		outline: none;
	}

	.uppercase {
		text-transform: uppercase;
	}

	.flat {
		box-shadow: none;
	}
</style>

<script>
	import { createEventDispatcher } from 'svelte';
	import Ripple from '../ripple/Ripple.svelte';

	export let uppercase = true;
	export let label = 'Click me';
	export let icon = null;
	export let showHTML = false;
	export let flat = true;

	let element;

	const dispatch = createEventDispatcher();

	function click(e) {
		dispatch('click', e);
	}

	function hover(e) {
		dispatch('hover', e);
	}
</script>

<Ripple hideOverflow={false}>
	<button on:click={click} on:hover={hover} class:uppercase class:flat>
		{#if icon != null}
			{#if typeof icon == 'string' && showHTML}
				{@html icon}
			{:else if icon == 'string'}
				{icon}
			{:else if typeof icon == 'element'}
				<div bind:this={element} />
			{:else}
				<svelte:component this={icon} />
			{/if}
		{/if}
		{#if showHTML}
			{@html label}
		{:else}{label}{/if}
		<slot>Some text</slot>
	</button>
</Ripple>
