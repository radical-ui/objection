<script>
	import { createEventDispatcher } from 'svelte';

	export let checked = false;
	export let partial = false;
	export let hovering = false;
	export let focused = false;
	export let disabled = false;

	const dispatch = createEventDispatcher();

	function handleInput() {
		partial = false;
	}

	function handleMouseover(e) {
		dispatch('hover', e);
		hovering = true;
	}

	function handleMouseout() {
		hovering = false;
	}
</script>

<style>
	.over {
		width: 40px;
		height: 40px;
		position: relative;
		border-radius: 50%;
		overflow: hidden;
		text-align: center;
	}
	input {
		position: absolute;
		top: -10px;
		right: -10px;
		width: 60px;
		height: 60px;
		opacity: 0;
		margin: 0;
	}
	.checkbox {
		line-height: 40px;
	}
</style>

<div
	class="over s-toolbox-checkbox"
	class:s-toolbox-checkbox-other={!disabled && !hovering && !focused}
	class:s-toolbox-checkbox-disabled={disabled}
	class:s-toolbox-checkbox-hover={hovering}
	class:s-toolbox-checkbox-focus={focused}
	on:mouseover={handleMouseover}
	on:mouseout={handleMouseout}>
	<input type="checkbox" bind:checked on:input={handleInput} />

	{#if checked}
		<i class="material-icons checkbox">check_box</i>
	{:else if partial}
		<i class="material-icons checkbox">indeterminate_check_box</i>
	{:else}
		<i class="material-icons checkbox">check_box_outline_blank</i>
	{/if}
</div>
