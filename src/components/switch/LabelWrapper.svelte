<script>
    export let label;
    export let left;
</script>

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