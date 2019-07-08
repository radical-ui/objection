<script>
	import Ripple from '../ripple/Ripple.svelte';
	import { createEventDispatcher } from 'svelte';

	export let on = false;
	export let onColor = 'var(--switch-on-color)';
	export let offColor = 'var(--switch-off-color)';
	export let onColorTrack = 'var(--switch-on-color-track)';
	export let offColorTrack = 'var(--switch-off-color-track)';
	export let activeColor = 'var(--switch-active-color)';
	export let activeOnColor = 'var(--switch-on-active-color)';
	export let hoverColor = 'var(--switch-hover-color)';
	export let hoverOnColor = 'var(--switch-hover-on-color)';
	export let shouldRipple = true;
	export let disabled = false;

	let hovering = false;
	let handleHovering = false;
	let active = false;
	let innerClicked = false;

	const dispatch = createEventDispatcher();

	function handleChange(e) {
		active = true;
		dispatch('change', e);
	}
	function handleMouseover(e) {
		hovering = true;
	}
	function handleMouseleave(e) {
		hovering = false;
	}
	function outsideClick(e) {
		if (!innerClicked) active = false;
		else innerClicked = false;
	}
</script>

<style>
	.over {
		--height: 48px;
		display: inline-block;
		width: 68px;
		height: var(--height);
		position: relative;
		cursor: pointer;
	}
	.track {
		width: 32px;
		height: 14px;
		border-radius: 7px;
		margin: 17px 18px;
	}
	.thumb {
		position: absolute;
		height: var(--height);
		width: var(--height);
		transition: left 200ms ease-out, background 0.2s;
		z-index: 1;
		overflow: hidden;
	}
	.thumb-inner {
		--margin: 14px;
		margin: var(--margin);
		width: calc(var(--height) - var(--margin) * 2);
		height: calc(var(--height) - var(--margin) * 2);
		box-shadow: 0px 3px 1px -2px rgba(0, 0, 0, 0.2),
			0px 2px 2px 0px rgba(0, 0, 0, 0.14),
			0px 1px 5px 0px rgba(0, 0, 0, 0.12);
	}
	input {
		display: none;
	}

	.round {
		border-radius: 50%;
	}
	.switch-disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.cover {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 2;
	}
</style>

<svelte:window on:click={outsideClick} />

<label>
	<div
		class="over"
		on:click={(_) => (innerClicked = true)}
		on:mouseover={handleMouseover}
		on:mouseleave={handleMouseleave}
		class:switch-on={on}
		class:switch-off={!on}
		class:switch-disabled={disabled}>
		<div
			class="thumb round"
			style="left:{on ? '20px' : '0px'}; background: {active ? (on ? activeOnColor : activeColor) : handleHovering ? (on ? hoverOnColor : hoverColor) : 'none'}"
			on:mouseover={(_) => (handleHovering = true)}
			on:mouseleave={(_) => (handleHovering = false)}>
			<Ripple
				disabled={!shouldRipple}
				color={on ? onColorTrack : offColorTrack}
				spread={100}
				time={200}
				center={true}>
				<div
					class="thumb-inner round"
					style="background: {on ? onColor : offColor}" />
			</Ripple>
		</div>
		<div
			class="track"
			style="background: {on ? onColorTrack : offColorTrack}" />
		<input
			type="checkbox"
			{disabled}
			on:change={handleChange}
			bind:checked={on} />
		{#if disabled}
			<div class="cover" />
		{/if}
	</div>
</label>
