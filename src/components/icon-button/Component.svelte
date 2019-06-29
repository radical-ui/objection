<script>
	import Ripple from '../ripple/Ripple.svelte';
	import { createEventDispatcher } from 'svelte';

	export let icon = 'error';
	export let disabled = false;
	export let on = false;
	export let activeColor = 'var(--icon-buttons-active)';
	export let hoverColor = 'var(--icon-buttons-hover)';
	export let rippleColor = 'var(--icon-buttons-ripple)';

	let toggle = typeof icon == 'array';
	let detail = typeof icon == 'object';
	let simple = typeof icon == 'string';

	let defaultColor = 'var(--icon-buttons)';

	let active = false;
	let hovering = false;
	let insideClicked = false;
	let backgroundColor = 'rgba(0, 0, 0, 0)';

	const dispatch = createEventDispatcher();

	function handleMouseover(e) {
		dispatch('hover', e);
		hovering = true;
	}

	function handleClick(e) {
		dispatch('click', e);
		insideClicked = true;
		active = true;
	}

	function handleMouseout(e) {
		hovering = false;
	}

	function handleOutsideClick(e) {
		if (!insideClicked) active = false;
		else insideClicked = false;
	}
</script>

<style>
	button {
		border: none;
		border: none;
		background: rgba(0, 0, 0, 0);
		outline: none;
		cursor: pointer;
		margin: 0;
		padding: 0;
		position: relative;
		z-index: 1;
		padding: 10px;
		transition: background 300ms;
	}
	.wrapper {
		overflow: hidden;
		border-radius: 50%;
		display: inline-block;
	}
</style>

<svelte:window on:click={handleOutsideClick} />

<div class="wrapper">
	<Ripple
		time={400}
		spread={100}
		hideOverflow={false}
		color={rippleColor}>
		<button
			style="background: {active ? activeColor : hovering ? hoverColor : backgroundColor}"
			on:mouseover={handleMouseover}
			on:mouseout={handleMouseout}
			on:click={handleClick}>

			{#if simple}
				<i class="material-icons" style="color: {defaultColor}">
					{icon}
				</i>
			{:else if detail}
				<i class="material-icons" style="color: {defaultColor}">
					{icon}
				</i>
			{:else if toggle}
				<i class="material-icons" style="color: {defaultColor}">
					{icon}
				</i>
			{/if}

		</button>
	</Ripple>
</div>
