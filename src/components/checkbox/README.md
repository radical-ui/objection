# Checkbox

A checkbox component following Material Design's guidlines.

```html
<script>
	import { Checkbox } from 'svelte-toolbox';
</script>

<Checkbox /> I accept the terms of service.
```

# Props

| Name       | Type      | Default    | Description                                                                                                         |
| :--------- | :-------- | :--------- | :------------------------------------------------------------------------------------------------------------------ |
| `checked`  | `boolean` | `false`    | If the checkbox is checked or not.                                                                                  |
| `partial`  | `boolean` | `false`    | If `true` Checkbox will display a partialy checked box. Setting partial to `true` will also set checked to `false`. |
| `hovering` | `boolean` | _ReadOnly_ | Will be `true` if the checkbox is being hovered upon.                                                               |
| `focused`  | `boolean` | _ReadOnly_ | Will be `true` if the checkbox is in focus.                                                                         |

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
