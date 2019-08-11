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
	.primary,
	.label {
		display: inline-block;
		vertical-align: top;
	}
</style>

{#if showLabel}
	<div class="over">
		<div class="primary">
			<slot name="primary" />
		</div>

		<div class="label">
			<span on:click={() => dispatch('click')}>
				{#if label}
					{label}
				{:else}
					<slot name="label" />
				{/if}
			</span>
		</div>
	</div>
{:else}
	<slot name="primary" />
{/if}
