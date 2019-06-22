<script>
	import { Ripple, UIButton, UIInput } from './components/components.module.js';

	let value = 'John Smith';

	function log(e) {
		console.log(e)
	}
</script>

<style>
	.over {
		font-size: 1rem;
	}
	span {
		text-transform: none;
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
	<UIButton color="rgba(0, 0, 200, 0.6)" hoverColor="rgba(0, 0, 200, 0.1)">
		Cancel
	</UIButton>
	<UIButton on:click={(e) => console.log(e)}>Save</UIButton>

	<br />

	<UIButton raised={true}>
		Show the
		<i>details for Nerds</i>
	</UIButton>

	<UIButton primary={true}>Click me</UIButton>
	<br />
	<UIButton primary={true} raised={true}>Click me</UIButton>
	<br />
	<UIButton disabled={true}>Disabled</UIButton>

	<h5>Href testing</h5>
	<UIButton href="/foo">
		Should go to
		<span>
			<i>host</i>
			/foo
		</span>
	</UIButton>
	<br />
	<UIButton href="bar" disabled={true}>
		Should not go to
		<span>
			<i>host</i>
			/bar
		</span>
	</UIButton>
	<br />
	<UIButton href="https://svelte.dev">
		Should go to
		<span>https://svelte.dev</span>
	</UIButton>

	<h1>UIInput</h1>
	<UIInput bind:value placeholder="What is your name?" maxChars={20}/>
	<UIInput disabled={true} />
	<UIInput type='password' error="Password is incorrect" />
	<UIInput isFocused={true} />
	<UIInput allowEnter={true} rows={5} compress={true} />
	<UIInput rows={null} resize={true} />
	<UIInput showLabel={false} />
	<UIInput border="none" />
	<UIInput border="full" />
	<UIInput rows={10} error="Too many rows!" />

	<UIInput
		on:valuechanged={log}
		on:prevaluechanged={log}
		on:focus={log}
		on:blur={log}
		on:keypress={log}
	/>
</div>
