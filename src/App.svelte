<script>
	import { Ripple, Button } from './components/components.module.js';
</script>

<style>
	.over {
		font-size: 1rem;
	}
</style>

<div class="over">
	<br />
	<br />

	<Ripple>
		<p>Look at this! Boy is it nice!</p>
	</Ripple>

	<Button>Cancel</Button>
	<Button on:click={(e) => console.log(e)}>Save</Button>

	<br />

	<Button>
		Show the
		<i>details for Nerds</i>
	</Button>
</div>
