<script>
	export let hovering = false;
	export let checked = false;
	export let partial = false;

	function handleInput(e) {
		partial = false;
	}
</script>

<style>
	.over {
		width: 40px;
		height: 40px;
		position: relative;
		border-radius: 50%;
		overflow: hidden;
		text-align: center;
	}
	input {
		position: absolute;
		top: -10px;
		right: -10px;
		width: 60px;
		height: 60px;
		opacity: 0;
		margin: 0;
	}
	.checkbox {
		line-height: 40px;
	}
</style>

<div class="over">
	<input type="checkbox" bind:checked on:input={handleInput} />

	{#if checked}
		<i class="material-icons checkbox">check_box</i>
	{:else if partial}
		<i class="material-icons checkbox">indeterminate_check_box</i>
	{:else}
		<i class="material-icons checkbox">check_box_outline_blank</i>
	{/if}
</div>
