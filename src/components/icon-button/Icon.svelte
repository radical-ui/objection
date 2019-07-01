<script>
	export let name = 'error';
	export let href = null;
	export let style = '';
	export let color = 'var(--icon-buttons)';
	export let newTab = false;

	let cs;
	$: cs = `color: ${color}; ${style};`;
</script>

<style>
	i {
		margin: 10px;
	}
	a {
		overflow: auto;
		position: relative;
		padding: 100px 0;
	}
</style>

{#if href}
	<a {href} target={newTab ? '_blank' : undefined}>
		<i class="material-icons" style={cs}>{name}</i>
	</a>
{:else}
	<i class="material-icons" style={cs}>{name}</i>
{/if}
