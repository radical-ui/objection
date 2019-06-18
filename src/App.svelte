<script>
	import { Ripple, Button } from './components/components.module.js';
</script>

<style>
	.over {
		font-size: 1rem;
	}
</style>

<div class="over">

	<h1>Svelte Toolbox</h1>

	<h2>Ripple</h2>
	<Ripple block={true}>
		<p>Full width</p>
	</Ripple>

	<Ripple disabled={true}>Disabled</Ripple>
	<Ripple hideOverflow={false}>Visibile overflow</Ripple>

	<h2>Button</h2>
	<Button>Cancel</Button>
	<Button on:click={(e) => console.log(e)}>Save</Button>

	<br />

	<Button>
		Show the
		<i>details for Nerds</i>
	</Button>
</div>
