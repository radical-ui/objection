# Ripple

Causes a ripple animation when the user clicks on it.

```html
<script>
	import { Ripple } from 'svelte-toolbox';
</script>

<Ripple>
	Click here!
</Ripple>
```

# Props

| Name           | Type      | Default              | Description                                                                  |
| :------------- | :-------- | :------------------- | :--------------------------------------------------------------------------- |
| `color`        | `string`  | `rgba(0, 0, 0, 0.2)` | The color of the ripple.                                                     |
| `spread`       | `number`  | `100`                | The amount of pixels the ripple will span at it's widest.                    |
| `time`         | `number`  | `500`                | The number of miliseconds it takes for the ripple to disapear.               |
| `hideOverflow` | `boolean` | `true`               | Specifies if the overflow from the ripples should be `hidden` or `visibile`. |

# Events

| Name          | `event.detail` | Description                     |
| :------------ | :------------- | :------------------------------ |
| `rippleEnded` | _none_         | Fires every time a ripple ends. |
