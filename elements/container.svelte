<script lang="ts">
	import { type Container } from '~/element_type'
	import ElementIndex from './index.svelte'
	import { getCSSContext } from '~/theme'

	const {
		child,
		border,
		borderColor,
		borderRadius,
		childPosition = 'center',
		clip,
		color,
		height,
		padding,
		paddingBottom,
		paddingLeft,
		paddingRight,
		paddingTop,
		paddingX,
		paddingY,
		size,
		width,
	}: Container = $props()

	// Get CSS context
	const { sizeToCSS, colorToCSS } = getCSSContext()

	// Calculate padding values with precedence
	const paddingTopValue = sizeToCSS(paddingTop ?? paddingY ?? padding)
	const paddingBottomValue = sizeToCSS(paddingBottom ?? paddingY ?? padding)
	const paddingLeftValue = sizeToCSS(paddingLeft ?? paddingX ?? padding)
	const paddingRightValue = sizeToCSS(paddingRight ?? paddingX ?? padding)

	// Map childPosition to flexbox properties
	const getChildPositioning = (position: typeof childPosition) => {
		const positions = {
			top_left: { justifyContent: 'flex-start', alignItems: 'flex-start' },
			top_center: { justifyContent: 'flex-start', alignItems: 'center' },
			top_right: { justifyContent: 'flex-start', alignItems: 'flex-end' },
			center_left: { justifyContent: 'center', alignItems: 'flex-start' },
			center: { justifyContent: 'center', alignItems: 'center' },
			bottom_left: { justifyContent: 'flex-end', alignItems: 'flex-start' },
			bottom_center: { justifyContent: 'flex-end', alignItems: 'center' },
			bottom_right: { justifyContent: 'flex-end', alignItems: 'flex-end' },
		} as const
		return positions[position as keyof typeof positions] || positions.center
	}

	const positioning = getChildPositioning(childPosition)
</script>

<div
	style:display="flex"
	style:flex-direction="column"
	style:justify-content={positioning.justifyContent}
	style:align-items={positioning.alignItems}
	style:width={sizeToCSS(width ?? size) ?? '100%'}
	style:height={sizeToCSS(height ?? size) ?? '100%'}
	style:background-color={colorToCSS(color)}
	style:padding-top={paddingTopValue}
	style:padding-bottom={paddingBottomValue}
	style:padding-left={paddingLeftValue}
	style:padding-right={paddingRightValue}
	style:border-radius={sizeToCSS(borderRadius)}
	style:border-width={sizeToCSS(border)}
	style:border-style={border ? 'solid' : undefined}
	style:border-color={colorToCSS(borderColor)}
	style:overflow={clip ? 'hidden' : undefined}
>
	{#if child}
		<ElementIndex {...child} />
	{/if}
</div>
