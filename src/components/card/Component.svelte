<script>
	import Ripple from '../ripple/Ripple.svelte';
	import UIButton from '../button/Component.svelte';

	export let href = null;
	export let ripple = href ? true : false;
	export let elevation = 2;
</script>

<style>
	.card {
		border-radius: 4px;
		overflow: hidden;
		width: 300px;
		display: inline-block;
		vertical-align: top;
		box-sizing: border-box;
		margin: 4px;
	}
	.sh1 {
		box-shadow: 0 4px 8px 2px rgba(0, 0, 0, 0.2);
	}
	.sh2 {
		box-shadow: 0 8px 16px 4px rgba(0,0,0,0.2);
	}
	.sh3 {
		box-shadow: 0 15px 23px 8px rgba(0,0,0,0.2);
	}
	.padding {
		padding: 0 16px;
	}
	.actions {
		padding: 4px;
	}
</style>

<div class="card" class:sh1={elevation == 1} class:sh2={elevation == 2} class:sh3={elevation == 3}>
	<Ripple disabled={!ripple} block>
		<slot>
			<Ripple block>
				<div class="padding"><h3>Some Title</h3></div>
				<img src="https://placekitten.com/300/200" alt="Kitten" />
				<div class="padding">
					<p>I like this card lot. How about you?</p>
					<p>It's awesome!</p>
					<p>Yes, I agree!</p>
				</div>
			</Ripple>
			<div class="actions">
				<UIButton>Ignore</UIButton>
				<UIButton>Respond</UIButton>
			</div>
		</slot>
	</Ripple>
</div>
