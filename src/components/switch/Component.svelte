<script>
	import Ripple from '../ripple/Ripple.svelte';
	export let on = false;
	export let onColor = 'green';
	export let offColor = '#ddd';
	export let offColorTrack = '#aaa';
	export let onColorTrack = 'rgba(16, 112, 4, 0.5)';
	export let activeColor = 'rgba(16, 112, 4, 0.1)';
	export let hoverColor = 'rgba(0, 0, 0, 0.07)';
	export let hoverOnColor = 'rgba(16, 112, 4, 0.07)';
	export let shouldRipple = true;

	let hovering = false;
	let handleHovering = false;
	let active = false;
	let innerClicked = false;

	function handleClick(e) {
		innerClicked = true;
		on = !on;
		active = on && true;
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
	.background {
		width: 32px;
		height: 14px;
		border-radius: 7px;
		margin: 17px 18px;
	}
	.handle {
		position: absolute;
		height: var(--height);
		width: var(--height);
		transition: left 200ms ease-out, background 0.2s;
		z-index: 1;
		overflow: hidden;
	}
	.handle-inner {
		--margin: 14px;
		margin: var(--margin);
		width: calc(var(--height) - var(--margin) * 2);
		height: calc(var(--height) - var(--margin) * 2);
		box-shadow: 0px 3px 1px -2px rgba(0, 0, 0, 0.2),
			0px 2px 2px 0px rgba(0, 0, 0, 0.14),
			0px 1px 5px 0px rgba(0, 0, 0, 0.12);
	}

	.round {
		border-radius: 50%;
	}
</style>

<svelte:window on:click={outsideClick} />

<div
	class="over"
	on:click={handleClick}
	on:mouseover={handleMouseover}
	on:mouseleave={handleMouseleave}>
	<div
		class="handle round"
		style="left:{on ? '20px' : '0px'}; background: {active ? activeColor : handleHovering ? (on ? hoverOnColor : hoverColor) : 'none'}"
		on:mouseover={(_) => (handleHovering = true)}
		on:mouseleave={(_) => (handleHovering = false)}>
		<Ripple
			disabled={!shouldRipple}
			color={on ? onColorTrack : offColorTrack}
			spread={100}
			center={true}>
			<div
				class="handle-inner round"
				style="background: {on ? onColor : offColor}" />
		</Ripple>
	</div>
	<div
		class="background"
		style="background: {on ? onColorTrack : offColorTrack}" />
</div>
