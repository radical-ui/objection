<script>
	import { createEventDispatcher } from 'svelte';
	import HelperText from './HelperText.svelte';
	import InputLine from './InputLine.svelte';
	import InputOutline from './InputOutline.svelte';

	export let value = null;
	export let placeholder = null;
	export let helper = null;
	export let persistHelper = false;
	export let type = 'text';
	export let rows = 1;
	export let compress = true;
	export let color = 'var(--inputs)';
	export let background = 'var(--inputs-background)';
	export let backgroundHover = 'var(--inputs-background-hover)';
	export let backgroundFocus = 'var(--inputs-background-focus)';
	export let placeholderColor = 'var(--inputs-placeholder)';
	export let outlineColor = 'var(--inputs-outline)';
	export let outlineHover = 'var(--inputs-outline-hover)';
	export let outlined = false;
	export let resize = false;
	export let allowEnter = false;
	export let isFocused = false;
	export let maxChars = null;
	export let error = null;
	export let disabled = false;
	export let block = false;
	export let showErrorIcon = true;
	export let trim = false;
	export let leadingIcon = null;
	export let trailingIcon = null;
	export let leadingIconColor = 'var(--inputs-placeholder)';
	export let trailingIconColor = 'var(--inputs-placeholder)';
	export let leadingIconIsButton = false;
	export let trailingIconIsButton = false;
	export let leadingIconHovering = false;
	export let trailingIconHovering = false;

	const dispatch = createEventDispatcher();
	let hovering = false;

	function mouseover(e) {
		hovering = true;
	}
	function mouseleave(e) {
		hovering = false;
	}
	function focus() {}
	function blur() {}
</script>

<style>
	.over {
		width: 250px;
		display: inline-block;
	}
	.over.block {
		display: block;
	}
</style>

<div class="over" class:block>
	<div
		class:input-disabled={disabled}
		class:input-focused={isFocused}
		class:input-error={error}
		class:input-else={!error && !isFocused && !disabled}
		style="background-color: {outlined ? 'unset' : isFocused ? backgroundFocus : hovering ? backgroundHover : background};"
		on:mouseover={mouseover}
		on:mouseleave={mouseleave}>

		<label>
			{#if outlined}
				<InputOutline />
			{:else}
				<InputLine />
			{/if}
		</label>

	</div>
	{#if (helper && isFocused) || (helper && persistHelper)}
		<HelperText {helper} />
	{/if}
</div>
