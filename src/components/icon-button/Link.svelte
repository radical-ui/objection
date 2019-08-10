<script>
	export let href;
	export let newTab;
</script>

<style>
	a {
		overflow: auto;
		position: relative;
		padding: 100px 0;
	}
</style>

{#if href}
	<a {href} target={newTab ? '_blank' : null}>
		<slot />
	</a>
{:else}
	<slot />
{/if}
