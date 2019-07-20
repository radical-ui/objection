<script>
	export let label;
	export let left;
</script>

<style>
	div {
		overflow: auto;
	}
	.left {
		float: left;
	}
	.right {
		float: right;
		height: 48px;
	}
	.switch-text-container {
		line-height: 48px;
	}

	.switch-container {
		padding: 5px 20px;
		user-select: none;
	}
</style>

{#if label != null}
	<div class="switch-container">
		{#if !left}
			<span class="left switch-text-container">{label}</span>
			<span class="right">
				<slot />
			</span>
		{:else}
			<span class="right switch-text-container">{label}</span>
			<span class="left">
				<slot />
			</span>
		{/if}
	</div>
{:else}
	<slot />
{/if}
