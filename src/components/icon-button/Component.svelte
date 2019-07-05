<script>
	import Ripple from '../ripple/Ripple.svelte';
	import Icon from './Icon.svelte';
	import { createEventDispatcher } from 'svelte';

	export let icon = 'error';
	export let disabled = false;
	export let on = false;
	export let activeColor = 'var(--icon-buttons-active)';
	export let hoverColor = 'var(--icon-buttons-hover)';
	export let rippleColor = 'var(--icon-buttons-ripple)';

	let toggle = typeof icon == 'object' && icon[0] != undefined;
	let detail = typeof icon == 'object' && icon[0] == undefined;
	let simple = typeof icon == 'string';

	let active = false;
	let hovering = false;
	let insideClicked = false;
	let backgroundColor = 'rgba(0, 0, 0, 0)';

	function getIconDetail(icon) {
		let detailedIcon = {};
		const values = ['name', 'href', 'style', 'color'];
		values.forEach((element) => {
			if (icon[element] != undefined) {
				if (typeof icon[element] == 'string')
					detailedIcon[element] = icon[element];
				else
					throw new TypeError(
						`Unexpected type in 'icon': '${element}' IconButton expected a string, but recieved '${typeof icon[
							element
						]}'.`
					);
			}
		});
		return detailedIcon;
	}

	// Events
	const dispatch = createEventDispatcher();

	function handleMouseover(e) {
		dispatch('hover', e);
		hovering = true;
	}

	function handleClick(e) {
		dispatch('click', e);
		insideClicked = true;
		active = true;
		on = !on;
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
		color={rippleColor}
		center={true}>
		<button
			style="background: {active ? activeColor : hovering ? hoverColor : backgroundColor}"
			on:mouseover={handleMouseover}
			on:mouseout={handleMouseout}
			on:click={handleClick}>

			{#if simple}
				<Icon name={icon} />
			{:else if detail}
				<Icon {...getIconDetail(icon)} />
			{:else if toggle}
				{#if on}
					<Icon {...getIconDetail(icon[0])} />
				{:else}
					<Icon {...getIconDetail(icon[1])} />
				{/if}
			{/if}

		</button>
	</Ripple>
</div>
