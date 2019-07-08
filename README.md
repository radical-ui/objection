# Svelte Toolbox

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

```shell
npm i svelte-toolbox
```

Not all of these components are stable. Please see the [Component Status](#component-status) section.

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

| Component                                          | Status                                                                                                     |
| :------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| [Ripple](src/components/ripple/README.md)          | **Stable**, no breaking changes or new features are expected.                                              |
| [UIButton](src/components/button/README.md)        | **Stable**, in that no breaking chenges are expected, but new features are.                                |
| [UIInput](src/components/input/README.md)          | **Mostly Stable**, there is some improvment under the hood to be done. This _might_ effect the public API. |
| [IconButton](src/components/icon-button/README.md) | **Stable**. Although new features are expected, no breaking changes are.                                   |
| [Switch](src/components/switch/README.md)          | **Unfinished**. This component is still in progress.                                                       |

### Documentation

Some of these components are still unstable. Please see the [Component Status](#component-status) section.

-   [Ripple](src/components/ripple/README.md)
-   [UIButton](src/components/button/README.md)
-   [UIInput](src/components/input/README.md)
-   [IconButton](src/components/icon-button/README.md)
-   [Switch](src/components/switch/README.md)

## Credits

-   [@YogliB](https://github.com/YogliB) for providing the [svelte-component-template](https://github.com/YogliB/svelte-component-template) used for this project
-   [@TehShrike](https://github.com/TehShrike) for his help and advice

## Inspiration

As I was working on an app using [Sapper](http://sapper.dev), I was made made aware of the fact that if there was a UI component library out there for [Svelte](http://svelte.dev), it would make developing a Svelte app so much easier!

I am a big fan of the Google Material Design patterns, and because I really like [React Toolbox](https://github.com/react-toolbox/react-toolbox), I decided to make something like it for Svelte.

## License

[MIT](LICENSE)
