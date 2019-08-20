<script>
	export let value;

	function focus() {}
	function blur() {}
</script>

<style>
	input {
		display: block;
		width: 100%;
		outline: none;
		border: none;
		font-size: 16px;
	}
</style>

<div class="over">
	<input type="text" bind:value on:focus={focus} on:blur={blur} />
</div>