<script>
	import { tweened } from 'svelte/motion';
	import { onMount, createEventDispatcher } from 'svelte';

	export let x;
	export let y;
	export let top;
	export let left;
	export let time;
	export let color;
	export let spread;
	export let center;
	export let height;
	export let width;

	const dispatch = createEventDispatcher();

	const opacity = tweened(1, {
		duration: time,
	});
	const size = tweened(0, {
		duration: time,
	});

	onMount(() => {
		opacity.set(0);
		size.set(spread);

		let done = false;
		opacity.subscribe((val) => {
			if (done && val == 0) finish();
			else done = true;
		});
	});

	function finish() {
		dispatch('finished');
	}
</script>

<style>
	div {
		position: absolute;
		opacity: 1;
		border-radius: 50%;
	}
</style>

<div
	class="ripple"
	style="background: {color}; opacity: {$opacity}; width: {$size}px; height: {$size}px;
	top: {center ? height / 2 - $size / 2 : y - $size / 2 - top}px; left: {center ? width / 2 - $size / 2 : x - $size / 2 - left}px;" />
