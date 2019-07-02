# Switch

An easy and user friendly expansion of `<input type="checkbox">`. [Demo]()

```html
<script>
	import { Switch } from 'svelte-toolbox';
	let on = false;
</script>

<Switch bind:on />

The switch is {on ? 'on' : 'off'}.
```

## Props

| Name            | Type      | Default                         | Description                                                      |
| :-------------- | :-------- | :------------------------------ | :--------------------------------------------------------------- |
| `on`            | `boolean` | `false`                         | If the switch is on or off                                       |
| `onColor`       | `string`  | `var(--switch-on-color)`        | The color of the switch thumb when the switch is on.             |
| `offColor`      | `string`  | `var(--switch-off-color)`       | Same as `onColor` except when the switch is off.                 |
| `onColorTrack`  | `string`  | `var(--switch-on-color-track)`  | Same as `offColorTrack` except when the switch is off.           |
| `offColorTrack` | `string`  | `var(--switch-off-color-track)` | The color of the track when switch is off.                       |
| `activeColor`   | `string`  | `var(--switch-active-color)`    | The color of the thumb container when the switch is active.      |
| `hoverColor`    | `string`  | `var(--switch-hover-color)`     | The color of the thumb container on hover.                       |
| `hoverOnColor`  | `string`  | `var(--switch-hover-on-color)`  | The color of the thumb container on hover when the switch is on. |
| `shouldRipple`  | `boolean` | `true`                          | If the Switch should display a ripple effect on click.           |

## Events

| Name     | `event.detail` | Description                                 |
| :------- | :------------- | :------------------------------------------ |
| `change` | _none_         | Fires every time the value of `on` changes. |

## CSS Selectors

_none_
