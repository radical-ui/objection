# IconButton

A Material Design style icon button.

```html
<script>
	import { IconButton } from 'svelte-toolbox';
</script>

<IconButton icon="favorite" />
```

# Props

If the `icon` is a string, or an object of type [`IconObject`](#IconObject), IconButton will display that icon; however, if icon is an array of objects of type [`IconObject`](#IconObject), IconButton will become a toggle button, and display `icon[0]` for on, and `icon[1]` for off.

| Name          | Type                                      | Default                      | Description                                                                                        |
| :------------ | :---------------------------------------- | :--------------------------- | :------------------------------------------------------------------------------------------------- |
| `icon`        | `string | IconObject | Array<IconObject>` | `error`                      | Specifies if the button should hav a ripple effect on click.                                       |
| `disabled`    | `boolean`                                 | `false`                      | This prop, as can be guessed, disables the button.                                                 |
| `on`          | `boolean`                                 | `false`                      | Will be true if the toggle is on. Only valid if `icon` is an array of [`IconObjects`](#IconObject) |
| `activeColor` | `string`                                  | `var(--icon-buttons-active)` | This is the color of the IconButton background after it has been clicked.                          |
| `hoverColor`  | `string`                                  | `var(--icon-buttons-hover)`  | The color of the IconButton background on hover.                                                   |
| `rippleColor` | `string`                                  | `var(--icon-buttons-ripple)` | The color of the ripple that occurs on click.                                                      |

## IconObject

| Name     | Type      | Default               | Description                                                               |
| :------- | :-------- | :-------------------- | :------------------------------------------------------------------------ |
| `name`   | `string`  | `error`               | The name of the icon.                                                     |
| `style`  | `string`  | `null`                | Icon style. This will be passed directly to the `style` attr on the icon. |
| `color`  | `string`  | `var(--icon-buttons)` | The color of the icon.                                                    |
| `href`   | `string`  | `null`                | Location to go to when icon is clicked.                                   |
| `newTab` | `boolean` | `false`               | Open the new location in a new tab. Only valid if `href != null`.         |

### Example

Here is an example using the `IconObject`:

```html
<IconButton
	icon="{{
	href: 'https://svelte.dev',
	style: 'padding: 10px',
	name: 'help'
}}"
/>
<!-- or -->
<IconButton
	icon="{[{
	name: 'favorite'
},
{
	name: 'favorite_outline'
}]}"
/>
```

# Events

| Name    | `event.detail` | Description                                 |
| :------ | :------------- | :------------------------------------------ |
| `click` | _`MouseEvent`_ | Fires when the user clicks the button.      |
| `hover` | _`MouseEvent`_ | Fires when the user hovers over the button. |
