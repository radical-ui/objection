<script>
	import { createEventDispatcher } from 'svelte';
	import Ripple from '../ripple/Ripple.svelte';
	import AWrapper from './AWrapper.svelte';

	export let raised = false;
	export let primary = false;
	export let outlined = false;
	export let transition = 200;
	export let ripple = true;
	export let block = false;
	export let disabled = false;
	export let color = 'var(--buttons)';
	export let textColor = 'var(--primary-buttons-text-color)';
	export let hoverColor = `var(--buttons-hover-color)`;
	export let primaryHoverColor = `var(--primary-buttons-hover-color)`;
	export let uppercase = true;
	export let rippleColor = `var(--buttons-ripple-color)`;
	export let primaryRippleColor = `var(--primary-buttons-ripple-color)`;
	export let href = null;

	let element;
	let hovering = false;

	const dispatch = createEventDispatcher();

	function click(e) {
		dispatch('click', e);
	}

	function hover(e) {
		dispatch('hover', e);
	}
</script>

<style>
	button {
		font-size: 1.2rem;
		padding: 0;
		margin: 0;
		line-height: 2rem;
		font-weight: 500;
		font-family: Roboto, Helvetica, Arial, sans-serif;

		border: none;
		background: rgba(0, 0, 0, 0);
		outline: none;
		border-radius: 3px;
		overflow: hidden;
	}
	button:not(.s-toolbox-ui-button-disabled) {
		cursor: pointer;
	}

	.s-toolbox-uppercase {
		text-transform: uppercase;
	}
	.s-toolbox-ui-button-disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.s-toolbox-raised {
		box-shadow: 0 2px 2px 0 rgba(0, 0, 0, 0.14),
			0 3px 1px -2px rgba(0, 0, 0, 0.2), 0 1px 5px 0 rgba(0, 0, 0, 0.12);
	}
	.block {
		display: block;
	}
	.up {
		position: relative;
		top: 0.8px;
	}
</style>

<AWrapper {href} {disabled}>
	<button
		class="s-toolbox-ui-button"
		on:click={click}
		on:mouseover={(e) => {
			hovering = true;
			hover();
		}}
		on:mouseout={(e) => (hovering = false)}
		class:s-toolbox-raised={raised}
		class:s-toolbox-block={block}
		class:s-toolbox-uppercase={uppercase}
		class:s-toolbox-ui-button-disabled={disabled}
		class:s-toolbox-ui-button-hovering={hovering}
		class:s-toolbox-ui-button-else={!disabled && !hovering}
		class:button-disabled={disabled}
		{disabled}
		style="{primary ? `background: ${hovering ? primaryHoverColor : color}; color: ${textColor}` : `color: ${color}; background: ${hovering ? hoverColor : 'rgba(0, 0, 0, 0)'}`};
		{outlined ? `border: 2px solid ${color};` : ''} transition: opacity {transition}ms,
		background {transition}ms;">

		<Ripple
			disabled={!ripple || disabled}
			block={true}
			time={600}
			color={primary ? primaryRippleColor : rippleColor}>
			<div style="padding: 0 12px;">
				<span class="up">
					<slot>Some text</slot>
				</span>
			</div>
		</Ripple>
	</button>
</AWrapper>
