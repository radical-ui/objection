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
	.disabled {
		opacity: 0.4;
	}
	.rounded {
		border-radius: 3px;
	}
	
	.switch-container {
		max-width: 800px;
		margin: auto;
		padding: 5px 20px;
		background-color: rgba(0, 0, 0, 0.1);
		border-bottom: 0.1px solid rgba(0, 0, 0, 0.2)
	}
</style>

{#if label != null}
    <div class="switch-container">
        {#if !left}
            <span class="left switch-text-container">{label}</span>
            <span class="right">
                <slot></slot>
            </span>
        {:else}
            <span class="right switch-text-container">{label}</span>
            <span class="left">
                <slot></slot>
            </span>
        {/if}
    </div>
{:else}
    <slot></slot>
{/if}