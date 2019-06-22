# UIInput

An input/textarea component with an animated bottom border.

```html
<script>
	import { UIInput } from 'svelte-toolbox';

	let value = 'Jack Smith';
	let placeholder = 'Enter a bio';

	function handleKeyUp(e) {
		console.log(e.target.value);
	}
	function handleSubmit(e) {
		alert('Form Submitted!');
	}
</script>

<form on:submit="{handleSubmit}">
	<UIInput bind:value />

	<p>Is your name {value}?</p>

	<UIInput rows="{10}" {placeholder} on:keyup="{handleKeyUp}" />
</form>
```

## Props

| Name               | Type      | Default                     | Description                                                                                                                                           |
| :----------------- | :-------- | :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`            | `string`  | `null`                      | The current value of the input.                                                                                                                       |
| `placeholder`      | `string`  | `null`                      | The placeholder and label of the input.                                                                                                               |
| `type`             | `string`  | `text`                      | The type of the input.                                                                                                                                |
| `rows`             | `number`  | `1`                         | The number of rows in the input.                                                                                                                      |
| `compress`         | `boolean` | `true`                      | If the input should have no more lines than needed. Only valid if `rows >= 2`.                                                                        |
| `color`            | `string`  | `var(--inputs)`             | The color of the text and the bottom border.                                                                                                          |
| `background`       | `string`  | `var(--inputs-background)`  | The background of the input.                                                                                                                          |
| `placeholderColor` | `string`  | `var(--inputs-placeholder)` | The color of the placeholder.                                                                                                                         |
| `showLabel`        | `boolean` | `true`                      | Specifies if the input should have a label or not.                                                                                                    |
| `animateLabel`     | `boolean` | `true`                      | Specifies if the input label should animate or not.                                                                                                   |
| `border`           | `string`  | `floor`                     | How the border should look. Valid values are `floor`, `full`, or `none`.                                                                              |
| `animateFloor`     | `boolean` | `true`                      | Specifies if the bottom border should animate or not. Only valid if `border = 'floor'`.                                                               |
| `resize`           | `boolean` | `false`                     | If the user should be allowed to resize the input. Only valid if `rows >= 2`.                                                                         |
| `allowEnter`       | `boolean` | `false`                     | If `true`, UIInput will call `event.preventDefault()` every time enter is pressed, and the user must press `shift` + `enter` to go to the next line. |
| `isFocused`        | `boolean` | `false`                     | If the input is focused or not.                                                                                                                       |
| `maxChars`         | `number`  | `null`                      | If a value is given, UIInput will not allow more characters than the given value.                                                                     |
| `error`            | `string`  | `null`                      | If a value is given, show an error underneath the input.                                                                                              |
| `disabled`         | `boolean` | `false`                     | This prop, as can be guessed, disables the button.                                                                                                    |

## Events

| Name              | `event.detail`     | Description                                                  |
| :---------------- | :----------------- | :----------------------------------------------------------- |
| `valuechanged`    | the new value      | Fires every time the value changes.                          |
| `prevaluechanged` | current value      | Fires every time the value is about to change.               |
| `focus`           | _none_             | Fires every time the input is focused.                       |
| `blur`            | _none_             | Fires every time the input is unfocused.                     |
| `keypress`        | HTML keydown event | Fires everytime a key is pressed while th input is in focus. |

## Selectors

| State      | CSS3 Selector     |
| :--------- | :---------------- |
| `disabled` | `.input-disabled` |
| `focused`  | `.input-focused`  |
| `error`    | `.input-error`    |
| `blured`   | `.input-blurred`  |

Note: Those CSS selectors only refrence the div that wraps the input. To style the input directly, add a `>input` to the end of each selector.
