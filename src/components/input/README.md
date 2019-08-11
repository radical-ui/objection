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

| Name                                        | Type      | Default                          | Description                                                                                                                                                                                                             |
| :------------------------------------------ | :-------- | :------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                                     | `string`  | `null`                           | The current value of the input.                                                                                                                                                                                         |
| `placeholder`                               | `string`  | `null`                           | The placeholder and label of the input.                                                                                                                                                                                 |
| `helper`                                    | `string`  | `null`                           | If specified, UIInput will display a helper text under the input.                                                                                                                                                       |
| `persistHelper`                             | `boolean` | `false`                          | If `true` UIInput will always show the helper even if the input is not is focus. Only valid if `helper != null`.                                                                                                        |
| `type`                                      | `string`  | `text`                           | The type of the input.                                                                                                                                                                                                  |
| `rows`                                      | `number`  | `1`                              | The number of rows in the input.                                                                                                                                                                                        |
| `compress`                                  | `boolean` | `true`                           | If the input should have no more lines than needed. Only valid if `rows >= 2`.                                                                                                                                          |
| `color`                                     | `string`  | `var(--inputs)`                  | The color of the text and the borders.                                                                                                                                                                                  |
| `background`                                | `string`  | `var(--inputs-background)`       | The background of the input.                                                                                                                                                                                            |
| `backgroundHover`                           | `string`  | `var(--inputs-background-hover)` | The background of the input on hover.                                                                                                                                                                                   |
| `backgroundFocus`                           | `string`  | `var(--inputs-background-focus)` | The color of the background when input is focused.                                                                                                                                                                      |
| `placeholderColor`                          | `string`  | `var(--inputs-placeholder)`      | The color of the placeholder and the bottom line.                                                                                                                                                                       |  |
| `outlineColor`                              | `string`  | `var(--inputs-outline)`          | The color of the outline when input is blured. Note: The color of the outline will be `color` when input is focused.                                                                                                    |
| `outlineHover`                              | `string`  | `var(--inputs-outline-hover)`    | The color of the outline on hover.                                                                                                                                                                                      |
| `outlined`                                  | `boolean` | `false`                          | If UIInput should show the outlined style.                                                                                                                                                                              |
| `resize`                                    | `boolean` | `false`                          | If the user should be allowed to resize the input. Only valid if `rows >= 2`.                                                                                                                                           |
| `allowEnter`                                | `boolean` | `false`                          | If `true`, UIInput will call `event.preventDefault()` every time enter is pressed, and the user must press `shift` + `enter` to go to the next line.                                                                    |
| `isFocused`                                 | `boolean` | `false`                          | If the input is focused or not.                                                                                                                                                                                         |
| `maxChars`                                  | `number`  | `null`                           | If a value is given, UIInput will not allow more characters than the given value.                                                                                                                                       |
| `error`                                     | `string`  | `null`                           | If a value is given, show an error underneath the input.                                                                                                                                                                |
| `disabled`                                  | `boolean` | `false`                          | This prop, as can be guessed, disables the button.                                                                                                                                                                      |
| `block`                                     | `boolean` | `true`                           | Specifies if the input should take up all available space.                                                                                                                                                              |
| `showErrorIcon`                             | `boolean` | `true`                           | Specifies if the `error` icon should be shown if `error != null`.                                                                                                                                                       |
| `trim`                                      | `boolean` | `false`                          | If `true` UIInput will remove the extra space that `helper`, `error`, and `maxChars` use. Warning: Only set this value to `true` if you will not be using the `helper`, `error`, and `maxChars` props on that instance. |
| `leadingIcon`                               | `string`  | `null`                           | If a value is given, UIInput will put the specified icon in front of the input. For a list of all valid values, see [here](https://material.io/tools/icons/?style=baseline).                                            |
| `trailingIcon`                              | `string`  | `null`                           | Same as `leadingIcon` except that is works on the trailing icon. If `error != null`, `trailingIcon` will be replaced with an error icon, unless `showErrorIcon == false`.                                               |
| `leadingIconColor`                          | `string`  | `var(--inputs-placeholder)`      | The color of the icon. For additional styling options, see [Selectors](#selectors).                                                                                                                                     |
| `trailingIconColor`                         | `string`  | `var(--inputs-placeholder)`      |                                                                                                                                                                                                                         |
| `leadingIconIsButton`                       | `boolean` | `false`                          | If `true` UIInput will treat the icon like a button.                                                                                                                                                                    |
| `trailingIconIsButton`                      | `boolean` | `false`                          |                                                                                                                                                                                                                         |
| `leadingIconIsHovering`<br />_(readOnly)_   | `boolean` | `false`                          | This prop will be `true` if the user is hovering over the icon and is `leadingIconIsButton == true`.                                                                                                                    |
| `ltrailingIconIsHovering`<br />_(readOnly)_ | `boolean` | `false`                          |                                                                                                                                                                                                                         |

## Events

| Name                | `event.detail`       | Description                                                           |
| :------------------ | :------------------- | :-------------------------------------------------------------------- |
| `valuechanged`      | the new value        | Fires every time the value changes.                                   |
| `prevaluechanged`   | current value        | Fires every time the value is about to change.                        |
| `focus`             | _none_               | Fires every time the input is focused.                                |
| `blur`              | _none_               | Fires every time the input is unfocused.                              |
| `keypress`          | HTML keydown event   | Fires every time a key is pressed while the input is in focus.        |
| `done`              | `value`              | Fires every time enter is pressed when `allowEnter == false`.         |
| `leadingiconhover`  | HTML mouseover event | Fires when the icon is hovered over if `leadingIconIsButton == true`. |
| `trailingiconhover` |
| `leadingiconclick`  | HTML click event     | Fires when the icon is clicked if `leadingIconIsButton == true`.      |
| `trailingiconclick` |

## Selectors

These CSS selectors only refrence the div that wraps the input.

| State             | CSS3 Selector     |
| :---------------- | :---------------- |
| `disabled`        | `.input-disabled` |
| `focused`         | `.input-focused`  |
| `error`           | `.input-error`    |
| _all other times_ | `.input-else`     |

### Sub-Selectors

Add the following suffix's the the above selectors to style the inner elements.

| Element           | Suffix           |
| :---------------- | :--------------- |
| The input element | `input`          |
| All icons         | `i`              |
| Leading icon      | `.leading-icon`  |
| Trailing icon     | `.trailing-icon` |
| Error icon        | `.error-icon`    |
| The input label   | `label`          |

#### Example:

```css
/* to hide all icons on a disabled UIInput */
.input-disabled i {
	display: none;
}
```
