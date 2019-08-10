<script>
	import { createEventDispatcher } from 'svelte';

	export let showLabel;
	export let label;

	const dispatch = createEventDispatcher();
</script>

<style>
	.over {
		position: relative;
	}
	.label {
		line-height: 40px;
		width: calc(100% - 50px);
		float: right;
	}
	.primary, .label {
		display: inline-block;
		vertical-align: top;
	}
</style>

{#if showLabel}
	<div class="over" on:click={() => dispatch('click')}>
		<div class="primary">
			<slot name="primary" />
		</div>
		
		<div class="label">
			{#if label}
				{label}
			{:else}
				<slot name="label" />
			{/if}
		</div>
	</div>
{:else}
	<slot name="primary" />
{/if}
