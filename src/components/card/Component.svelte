<script>
	import Ripple from '../ripple/Ripple.svelte';
	import UIButton from '../button/Component.svelte';
	import { createEventDispatcher } from 'svelte';

	export let href = null;
	export let ripple = href ? true : false;
	export let elevation = 2; // 1, 2, 3
	export let hovering = false;
	export let animateElevationSpeed = 500;
	export let outlined = false;
	export let backgroundColor = `var(--card-background-color)`;
	export let backgroundColorHover = `var(--card-background-color-hover)`;
	export let outlineColor = `var(--card-outline-color)`;
	export let outlineColorHover = `var(--card-outline-color-hover)`;
	export let corners = `rounded`; // `square`, `rounded`, `extra-rounded`, `sleek-right`, `sleek-left`, `standing`, `hanging`

	const dispatch = createEventDispatcher();

	let link;

	function handleClick(e) {
		dispatch('click', e);
		link.click();
	}

	function handleMouseover(e) {
		dispatch('hover', e);
		hovering = true;
	}

	function handleMouseout(e) {
		hovering = false;
	}
</script>

<style>
	.card {
		overflow: hidden;
		width: 300px;
		display: inline-block;
		vertical-align: top;
		box-sizing: border-box;
		margin: 4px;
	}

	.sh1 {
		box-shadow: 0 4px 8px 2px rgba(0, 0, 0, 0.2);
	}
	.sh2 {
		box-shadow: 0 8px 16px 4px rgba(0, 0, 0, 0.2);
	}
	.sh3 {
		box-shadow: 0 15px 23px 8px rgba(0, 0, 0, 0.2);
	}
	.padding {
		padding: 0 16px;
	}
	.actions {
		padding: 4px;
	}
	.outlined {
		box-shadow: 0 0 0 0 rgba(0, 0, 0, 0);
		border: 1px solid rgba(0, 0, 0, 0);
	}
	.rounded {
		border-radius: 4px;
	}
	.extra-rounded {
		border-radius: 10px;
	}
	.sleek-right {
		border-radius: 25px 0 25px 0;
	}
	.sleek-left {
		border-radius: 0 25px 0 25px;
	}
	.standing {
		border-radius: 18px 18px 0 0;
	}
	.hanging {
		border-radius: 0 0 18px 18px;
	}
	a {
		display: none;
	}
</style>

<div
	class="card s-toolbox-card {corners}"
	class:sh1={elevation == 1}
	class:sh2={elevation == 2}
	class:sh3={elevation == 3}
	class:outlined
	on:click={handleClick}
	on:mouseover={handleMouseover}
	on:mouseout={handleMouseout}
	style="background: {hovering ? backgroundColorHover : backgroundColor};
	border-color: {hovering ? outlineColorHover : outlineColor}; transition:
	box-shadow {animateElevationSpeed}ms, background 300ms;">
	<Ripple disabled={!ripple} card>
		<slot>
			<Ripple card>
				<div class="padding">
					<h3>Some Title</h3>
				</div>
				<img src="https://placekitten.com/300/200" alt="Kitten" />
				<div class="padding">
					<p>I like this card lot. How about you?</p>
					<p>It's awesome!</p>
					<p>Yes, I agree!</p>
				</div>
			</Ripple>
			<div class="actions">
				<UIButton>Ignore</UIButton>
				<UIButton>Respond</UIButton>
			</div>
		</slot>
	</Ripple>
</div>
<a {href} bind:this={link}>[link]</a>
