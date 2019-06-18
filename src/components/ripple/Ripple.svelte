<script>
	import Circle from './Circle.svelte';
	import { createEventDispatcher } from 'svelte';

	export let color = 'rgba(0, 0, 0, 0.2)';
	export let spread = 100;
	export let time = 500;
	export let hideOverflow = true;

	const dispatch = createEventDispatcher();

	let ripples = [];
	let container;

	function startRipple(e) {
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
	}
</style>

<div
	on:click={startRipple}
	bind:this={container}
	style="overflow: {hideOverflow ? 'hidden' : 'visibile'}">
	{#each ripples as { x, y, top, left, id }, index (id)}
		<Circle
			{x}
			{y}
			{top}
			{left}
			{color}
			{spread}
			{time}
			on:finished={(_) => {
				dispatch('rippleEnded');
				stopRipple(index, id);
			}} />
	{/each}
	<slot>Some text</slot>
</div>
