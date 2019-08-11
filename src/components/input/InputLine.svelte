<script>
	export let value;

	function focus() {}
	function blur() {}
</script>

<style>

</style>

<div class="over">
	<input type="text" bind:value on:focus={focus} on:blur={blur} />
</div>
