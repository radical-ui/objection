> The purpose of this tool was always to reduce the complexity of building highly interactive and beautiful user interfaces. To that end I've rebranded this project under the name of Objection. It no longer uses Svelte for various technical reasons, but that shouldn't matter, should it?
>
> You can find more at the [`next` branch](https://github.com/radical-ui/objection/blob/next).

[Documentation](#documentation) • [Changelog](./CHANGELOG.md) • [Component Status](#component-status)

# Svelte Toolbox ([demo](https://svelte.dev/repl/5cf847108884453cbedd5920e919b630?version=3.6.5))

A UI component library for Svelte implementing Google's Material Design specification.

Beacuse Svelte Toolbox is early in development, some things may change before we hit `v1.0.0` (Please see [Component Status](#component-status)).

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

Not all of these components are stable. Please see the [Component Status](#component-status) section.

There is detailed documentation about each of the components [below](#documentation), but the basic usage can bee seen below.

### Svelte/Sapper

Installing `svelte-toolbox` as a `devDependency` allows Svelte to compile the svelte-toolbox components right along with the rest of your code.

```shell
npm i --save-dev svelte-toolbox
# or yarn
```

```html
<script>
	import { UIButton, Ripple } from 'svelte-toolbox';
</script>

<UIButton on:click="{() => alert('done!')}">Click me!</UIButton>

<Ripple>
	There is a nice ripple effect on this text.
</Ripple>
```

### HTML/CSS/VanillaJS

You can use CDN's from `jsDelivr.net`, `unpkg.com`, or `bundle.run`. You can also install `svelte-toolbox` into your project via `npm` or `yarn`.

In this example, however, we will use the `unpkg` CDN:

```html
<!-- CSS/JS -->
<script src="https://unpkg.com/svelte-toolbox/dist/index.min.js"></script>
<link
	rel="stylesheet"
	href="https://unpkg.com/svelte-toolbox/public/bundle.css"
/>

<!-- Default styles -->
<link
	rel="stylesheet"
	href="https://unpkg.com/svelte-toolbox/public/global.css"
/>

<div id="button"></div>

<script>
	new Toolbox.UIButton({ target: document.querySelector('#button') });
</script>
```

### Demos

-   [Svelte](https://svelte.dev/repl/5cf847108884453cbedd5920e919b630?version=3.6.5)
-   [Sapper](https://codesandbox.io/s/github/svelte-toolbox/sapper-example)
-   [HTML/CSS/VanillaJS](https://jsfiddle.net/Vehmloewff/5rfdh0y2/64/)

### Global Styles

We recommend adding the contents of [`public/global.css`](public/global.css) to your global stylesheet. These will be the default styles for the components you import from `svelte-toolbox`.

P.S. If you like the styles in [`public/global.css`](public/global.css), and don't want to change them, you could just link to it:

```html
<link
	href="https://unpkg.com/svelte-toolbox/public/global.css"
	rel="stylesheet"
/>
<!-- or, depending on your setup, you might even be able to do this  -->
<link href="node_modules/svelte-toolbox/public/global.css" rel="stylesheet" />
```

### Component Status

| Component                                          | Status                                                                                                                                    |
| :------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| [Ripple](src/components/ripple/README.md)          | **Stable**, no breaking changes or new features are expected.                                                                             |
| [UIButton](src/components/button/README.md)        | **Stable**, in that no breaking chenges are expected, but new features are.                                                               |
| [UIInput](src/components/input/README.md)          | **Mostly Stable**, there is some improvment under the hood to be done. This _might_ effect the public API.                                |
| [IconButton](src/components/icon-button/README.md) | **Stable**. Although new features are expected, no breaking changes are.                                                                  |
| [Switch](src/components/switch/README.md)          | **Stable**. Although new features are expected, no breaking changes are.                                                                  |
| [Card](src/components/card/README.md)              | **Stable**, no breaking changes or new features are expected.                                                                             |
| [Checkbox](src/components/checkbox/README.md)      | **Unstable**. This component is not yet finished. Please see [this project](https://github.com/svelte-toolbox/svelte-toolbox/projects/1). |

### Documentation

Some of these components are still unstable. Please see the [Component Status](#component-status) section.

-   [Ripple](src/components/ripple/README.md)
-   [UIButton](src/components/button/README.md)
-   [UIInput](src/components/input/README.md)
-   [IconButton](src/components/icon-button/README.md)
-   [Switch](src/components/switch/README.md)
-   [Card](src/components/card/README.md)
-   [Checkbox](src/components/checkbox/README.md)

## Need help? Have a question?

Then you have come to the right place.

[Open an Issue](https://github.com/svelte-toolbox/svelte-toolbox/issues/new) or join the question friendly [Discord Server](https://discord.gg/bWZnuvdhttps://discord.gg/bWZnuvd).

## Credits

-   [@YogliB](https://github.com/YogliB) for providing the [svelte-component-template](https://github.com/YogliB/svelte-component-template) used for this project
-   [@TehShrike](https://github.com/TehShrike) for his help and advice

## Inspiration

As I was working on an app using [Sapper](http://sapper.dev), I was made aware of the fact that if there was a UI component library out there for [Svelte](http://svelte.dev), it would make developing a Svelte app so much easier!

I am a big fan of the Google Material Design patterns, and because I really like [React Toolbox](https://github.com/react-toolbox/react-toolbox), I decided to make something like it for Svelte.

## License

[MIT](LICENSE)
