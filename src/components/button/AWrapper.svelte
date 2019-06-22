<script>
	export let href;
	export let disabled;
</script>

{#if href && !disabled}
	<a {href}>
		<slot />
	</a>
{:else}
	<slot />
{/if}
