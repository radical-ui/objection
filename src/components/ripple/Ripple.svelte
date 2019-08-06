<script>
	import Circle from './Circle.svelte';
	import { createEventDispatcher } from 'svelte';

	export let color = 'var(--ripple-color)';
	export let spread = 300;
	export let time = 300;
	export let hideOverflow = true;
	export let disabled = false;
	export let block = false;
	export let center = false;
	export let card = false;

	const dispatch = createEventDispatcher();

	let ripples = [];
	let container;

	function startRipple(e) {
		if (disabled) return;
		ripples.push({
			x: e.clientX,
			y: e.clientY,
			top: container.getBoundingClientRect().top,
			left: container.getBoundingClientRect().left,
			id: new Date().getTime(),
		});
		update();
	}

	function stopRipple(index, id) {
		if (ripples[index].id == id) {
			ripples.splice(index, 1);
			update();
		} else ripples = ripples.filter((r) => r.id != id);
	}

	function update() {
		ripples = ripples;
	}
</script>

<style>
	div {
		position: relative;
		overflow: visible;
		display: inline-block;
	}
	.hideOverflow {
		overflow: hidden;
	}
	.block {
		display: block;
	}
	.card {
		width: 300px;
	}
</style>

<div
	on:click={startRipple}
	bind:this={container}
	class:hideOverflow
	class:block={block || card}
	class:card>
	{#each ripples as { x, y, top, left, id }, index (id)}
		<Circle
			{x}
			{y}
			{top}
			{left}
			{color}
			{spread}
			{time}
			{center}
			height={container.offsetHeight}
			width={container.offsetWidth}
			on:finished={(_) => {
				dispatch('rippleEnded');
				stopRipple(index, id);
			}} />
	{/each}
	<slot>Some text</slot>
</div>
