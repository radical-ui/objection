# Checkbox

A checkbox component following Material Design's guidlines.

```html
<script>
	import { Checkbox } from 'svelte-toolbox';
</script>

<Checkbox /> I accept the terms of service.
```

# Props

| Name             | Type      | Default                            | Description                                                                                                         |
| :--------------- | :-------- | :--------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| `checked`        | `boolean` | `false`                            | If the checkbox is checked or not.                                                                                  |
| `partial`        | `boolean` | `false`                            | If `true` Checkbox will display a partialy checked box. Setting partial to `true` will also set checked to `false`. |
| `hovering`       | `boolean` | _ReadOnly_                         | Will be `true` if the checkbox is being hovered upon.                                                               |
| `focused`        | `boolean` | _ReadOnly_                         | Will be `true` if the checkbox is in focus.                                                                         |
| `disabled`       | `boolean` | `false`                            | As can be guessed, this prop disables the checkbox.                                                                 |
| `color`          | `string`  | `var(--checkbox-color)`            | The color of the checkbox when it is on or when `parital == true`.                                                  |
| `colorOff`       | `string`  | `var(--checkbox-color-off)`        | The color of the checkbox when it is off.                                                                           |
| `hoverColor`     | `string`  | `var(--checkbox-hover-color)`      | The color of the checkbox background when hovered upon when it is on or when `parital == true`.                     |
| `hoverColorOff`  | `string`  | `var(--checkbox-hover-color-off)`  | The color of the checkbox background when hovered upon when it is off.                                              |
| `focusColor`     | `string`  | `var(--checkbox-focus-color)`      | The color of the checkbox background when in focus while it is on.                                                  |
| `focusColorOff`  | `string`  | `var(--checkbox-focus-color-off)`  | The color of the checkbox background when in focus while it is off.                                                 |
| `rippleColor`    | `string`  | `var(--checkbox-ripple-color)`     | The color of the ripple when the checkbox is on. Only valid if `shouldRipple == true`.                              |
| `rippleColorOff` | `string`  | `var(--checkbox-ripple-color-off)` | The color of the ripple when the checkbox is off. Only valid if `shouldRipple == true`.                             |

# Events

| Name    | `event.detail` | Description                                   |
| :------ | :------------- | :-------------------------------------------- |
| `click` | _`MouseEvent`_ | Fires when the user clicks the checkbox.      |
| `hover` | _`MouseEvent`_ | Fires when the user hovers over the checkbox. |

# CSS Selectors

| State      | Selector                       |
| :--------- | :----------------------------- |
| Disabled   | `.s-toolbox-checkbox-disabled` |
| Hover      | `.s-toolbox-checkbox-hover`    |
| Focus      | `.s-toolbox-checkbox-focus`    |
| Other      | `.s-toolbox-checkbox-other`    |
| all states | `.s-toolbox-checkbox`          |
