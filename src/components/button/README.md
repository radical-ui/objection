# Button

A Material Design style button.

```html
<script>
	import { UIButton } from 'svelte-toolbox';
</script>

<UIButton raised="{true}">Raised Button</UIButton>
<UIButton>Flat Button</UIButton>
<UIButton primary="{true}">Primary Button</UIButton>
```

# Props

| Name                | Type      | Default                              | Description                                                                                         |
| :------------------ | :-------- | :----------------------------------- | :-------------------------------------------------------------------------------------------------- |
| `raised`            | `boolean` | `false`                              | Specifies if the button is to have a box shadow on it.                                              |
| `primary`           | `boolean` | `false`                              | If the button should have a solid background color. This color can be styled with the `color` prop. |  |
| `transition`        | `number`  | `200`                                | The number of miliseconds allowed for the hover transition.                                         |
| `ripple`            | `boolean` | `true`                               | Specifies if the button should have a ripple effect on click.                                       |
| `block`             | `boolean` | `false`                              | If true, button will `display: block` on it.                                                        |
| `disabled`          | `boolean` | `false`                              | This prop, as can be guessed, disables the button.                                                  |
| `color`             | `string`  | `var(--buttons)`                     | This is the color of the button text or the background of the button if `primary` is `true`.        |
| `textColor`         | `string`  | `var(--primary-buttons-text-color)`  | This is the color of the button text when `primary` is `true`.                                      |
| `hoverColor`        | `string`  | `var(--buttons-hover-color)`         | The color of the button on hover. This will not work when `primary` is `true`.                      |
| `primaryHoverColor` | `string`  | `var(--primary-buttons-hover-color)` | The color of the button on hover when `primary` is `true`.                                          |
| `uppercase`         | `boolean` | `true`                               | Specifies if the button text should be uppercased.                                                  |
| `href`              | `string`  | `null`                               | Location to go to when button is clicked.                                                           |

# Events

| Name    | `event.detail` | Description                                 |
| :------ | :------------- | :------------------------------------------ |
| `click` | _`MouseEvent`_ | Fires when the user clicks the button.      |
| `hover` | _`MouseEvent`_ | Fires when the user hovers over the button. |
