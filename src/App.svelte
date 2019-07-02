<script>
	import {
		Ripple,
		UIButton,
		UIInput,
		IconButton,
	} from './components/components.module.js';

	let value = 'John Smith';
	let trailingIconHovering;

	function log(e) {
		console.log(e);
	}
</script>

<style>
	.over {
		font-size: 1rem;
		padding-bottom: 100px;
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
	<p>placeholder, maxChars</p>
	<UIInput bind:value placeholder="What is your name?" maxChars={20} />
	<p>placeholder, maxChars, helper</p>
	<UIInput
		bind:value
		placeholder="Name"
		helper="We respect your privacy"
		maxChars={20} />
	<p>disabled</p>
	<UIInput disabled={true} />
	<p>type = password, error</p>
	<UIInput type="password" error="Password is incorrect" />
	<p>isFocused = true</p>
	<UIInput isFocused={true} />
	<p>rows = 5, compress, allowEnter</p>
	<UIInput allowEnter={true} rows={5} compress={true} />
	<p>rows = null (input should get as big as it needs to), resize</p>
	<UIInput rows={null} resize={true} />
	<p>outlined, placeholder</p>
	<UIInput outlined={true} placeholder="Name or email" />
	<p>rows = 10, error, placeholder, !compress</p>
	<UIInput
		rows={10}
		error="Too many rows!"
		placeholder="Enter your thoughts"
		compress={false} />
	<p>placeholder, block</p>
	<UIInput placeholder="Enter name..." block={true} />
	<p>placeholder, outlined, rows == null, allowEnter, helper, maxChars</p>
	<UIInput
		placeholder="Username or email"
		outlined={true}
		rows={null}
		allowEnter={true}
		helper="We respect your privacy"
		maxChars={50} />

	<p>Icons:</p>
	<p>leadingIcon, placeholder</p>
	<UIInput placeholder="Input with leadingIcon" leadingIcon="favorite" />
	<p>trailingIcon, placeholder</p>
	<UIInput
		placeholder="Input with trailingIcon"
		trailingIcon="favorite_border"
		bind:trailingIconHovering
		trailingIconIsButton={true}
		trailingIconColor={trailingIconHovering ? 'red' : undefined}
		on:trailingiconclick={log}
		on:trailingiconhover={log} />
	<p>leadingIcon, trailingIcon, placeholder</p>
	<UIInput
		placeholder="Input w/ both"
		trailingIcon="done"
		leadingIcon="favorite" />
	<p>leadingIcon, trailingIcon, placeholder, outlined, value</p>
	<UIInput
		placeholder="Input w/ both"
		trailingIcon="done"
		leadingIcon="favorite"
		outlined={true}
		value="Good idea!" />

	<p>Events:</p>
	<UIInput
		on:valuechanged={log}
		on:prevaluechanged={log}
		on:focus={log}
		on:blur={log}
		on:keypress={log}
		on:done={log}
		on:leadingiconclick={log}
		on:leadingiconhover={log}
		on:trailingiconclick={log}
		on:trailingiconhover={log} />

	<h1>IconButton</h1>
	<IconButton />
	<IconButton icon="favorite" />
	<IconButton icon="play_arrow" />
	<IconButton
		icon={{ name: 'launch', color: 'green', href: 'https://svelte.dev', style: 'margin: 5px' }} />
	<IconButton
		icon={[{ name: 'favorite', color: 'green' }, { name: 'favorite_border', color: 'black' }]} />
</div>
