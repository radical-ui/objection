# Button

A simple Material Design style card.

```html
<script>
	import { Card } from 'svelte-toolbox';
</script>

<Card>
	<h2>Look at this card!</h2>
	<p>Wow! It's awesome!</p>
</Card>
```

# Props

| Name                    | Type      | Default               | Description                                                                                                                                                                                 |
| :---------------------- | :-------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `href`                  | `string`  | `null`                | Location to go to when the card is clicked.                                                                                                                                                 |
| `ripple`                | `boolean` | `href ? true : false` | If the card should ripple or not when clicked.<br />_Note: This will ripple over the whole card. To only ripple a part of the card use [Ripple](../ripple/README.md) with the `card` prop._ |
| `elevation`             | `number`  | `2`                   | A value between 1 & 3 specifing how much of a shadow the card will have.                                                                                                                    |
| `hovering`              | `boolean` | _ReadOnly_            | Wether the user is hovering over the card or not.                                                                                                                                           |
| `animateElevationSpeed` | `number`  | `500`                 | The number of ms it should take to transition between the levels of elevation.                                                                                                              |

# Events

| Name    | `event.detail` | Description                               |
| :------ | :------------- | :---------------------------------------- |
| `click` | _`MouseEvent`_ | Fires when the user clicks the card.      |
| `hover` | _`MouseEvent`_ | Fires when the user hovers over the card. |

# CSS Selectors

| State      | Selector          |
| :--------- | :---------------- |
| all states | `.s-toolbox-card` |
