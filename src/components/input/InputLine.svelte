<script>
	import Input from './Input.svelte';
	export let value;

	function focus() {}
	function blur() {}
</script>

<style>
	.over {
		padding: 10px;
	}
</style>

<div class="over">
	<Input {value} />
</div>
