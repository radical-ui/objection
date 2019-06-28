# Svelte Toolbox

A UI component library for Svelte implementing Google's Material Design specification.

Beacuse Svelte Toolbox is early in development, some things may change before we hit `v1.0.0`.

## Contributing

Pull requests are always welcome!

```bash
git clone https://github.com/svelte-toolbox/svelte-toolbox.git
cd svelte-toolbox
npm install
```

To start the development server:

```bash
npm run dev
```

To clean up the project and format the files:

```bash
npm run lint
```

You are welcome to add new features or components, but please open an issue to discuss your idea first.

## Usage

```shell
npm i svelte-toolbox
```

There is detailed documentation about each of the components [below](#documentation), but the basic usage looks like this:

```html
<script>
	import { UIButton, Ripple } from 'svelte-toolbox';
</script>

<UIButton on:click="{() => alert('done!')}">Click me!</UIButton>

<Ripple>
	There is a nice ripple effect on this text.
</Ripple>
```

### Global Styles

We recommend adding these lines to your global stylesheet. These will be the default styles for the components you import from `svelte-toolbox`.

```css
:root {
    /* buttons */
    --buttons: #303ba6;
    --primary-buttons-text-color: white;
    --buttons-hover-color: #303ca649;
	--primary-buttons-hover-color: var(--buttons);
	--buttons-ripple-color: #303ca69d;
	--primary-buttons-ripple-color: rgba(255, 255, 255, 0.5);

	/* inputs */
	--inputs: var(--buttons);
	--inputs-background: #ecebeb;
	--inputs-background-hover: #ebebeb;
	--inputs-background-focus: #e7e4e4;
	--inputs-placeholder: #696969;
	--inputs-outline: grey;
	--inputs-outline-hover: var(--inputs-outline);

	/* error */
	--all-errors: #b00020;
}
.button-disabled {
	opacity: 0.4;
	cursor: not-allowed;
}
```

### Documentation

-   [Ripple](https://github.com/svelte-toolbox/svelte-toolbox/tree/master/src/components/ripple/README.md)
-   [UIButton](https://github.com/svelte-toolbox/svelte-toolbox/tree/master/src/components/button/README.md)
-   [UIInput](https://github.com/svelte-toolbox/svelte-toolbox/tree/master/src/components/input/README.md)

## Inspiration

As I was working on an app using [Sapper](http://sapper.dev), I was made made aware of the fact that if there was a UI component library out there for [Svelte](http://svelte.dev), it would make developing a Svelte app so much easier!

I am a big fan of the Google Material Design patterns, and because I really like [React Toolbox](https://github.com/react-toolbox/react-toolbox), I decided to make something like it for Svelte.

## License

[MIT](https://github.com/svelte-toolbox/svelte-toolbox/blob/master/LICENSE)
