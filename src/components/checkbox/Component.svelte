<script>
	import { createEventDispatcher, afterUpdate } from 'svelte';
	import Ripple from '../ripple/Ripple.svelte';

	export let checked = false;
	export let partial = false;
	export let hovering = false;
	export let focused = false;
	export let disabled = false;
	export let color = `var(--checkbox-color)`;
	export let colorOff = `var(--checkbox-color-off)`;
	export let hoverColor = `var(--checkbox-hover-color)`;
	export let hoverColorOff = `var(--checkbox-hover-color-off)`;
	export let focusColor = `var(--checkbox-focus-color)`;
	export let focusColorOff = `var(--checkbox-focus-color-off)`;
	export let rippleColor = `var(--checkbox-ripple-color)`;
	export let rippleColorOff = `var(--checkbox-ripple-color-off)`;

	let insideClicked = false;

	const dispatch = createEventDispatcher();

	let style;
	$: style = `background: ${
		focused
			? checked
				? focusColor
				: focusColorOff
			: hovering
			? checked
				? hoverColor
				: hoverColorOff
			: 'none'
	}`;

	function handleInput() {
		partial = false;
	}

	function handleClick() {
		focused = true;
		insideClicked = true;
	}

	function largeClick() {
		if (!insideClicked) focused = false;
		else insideClicked = false;
	}

	function handleMouseover(e) {
		dispatch('hover', e);
		hovering = true;
	}

	function handleMouseout() {
		hovering = false;
	}

	afterUpdate(() => {
		if (partial && checked) checked = false;
	});
</script>

<style>
	.round {
		border-radius: 50%;
		overflow: hidden;
		width: 40px;
		height: 40px;
		display: inline-block;
	}
	.over {
		text-align: center;
		transition: background 0.3s;
		cursor: pointer;
		display: block;
	}
	input {
		display: none;
	}
	.checkbox {
		line-height: 40px;
	}
</style>

<svelte:window on:click={largeClick} />

<label>
	<div class="round">
		<div
			class="over s-toolbox-checkbox"
			class:s-toolbox-checkbox-other={!disabled && !hovering && !focused}
			class:s-toolbox-checkbox-disabled={disabled}
			class:s-toolbox-checkbox-hover={hovering}
			class:s-toolbox-checkbox-focus={focused}
			on:mouseover={handleMouseover}
			on:mouseout={handleMouseout}
			on:click={handleClick}
			{style}>
			<input type="checkbox" bind:checked on:input={handleInput} />

			<Ripple
				center
				width="40px"
				color={checked ? rippleColor : rippleColorOff}>
				{#if checked}
					<i class="material-icons checkbox" style="color: {color}">
						check_box
					</i>
				{:else if partial}
					<i class="material-icons checkbox" style="color: {color}">
						indeterminate_check_box
					</i>
				{:else}
					<i
						class="material-icons checkbox"
						style="color: {colorOff}">
						check_box_outline_blank
					</i>
				{/if}
			</Ripple>
		</div>
	</div>
</label>
