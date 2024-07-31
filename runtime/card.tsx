import { ComponentRender } from './component.tsx'
import { Component, React } from './deps.ts'
import { SkeletonBlock, useSkeletonDetection } from './skeleton.tsx'
import { ColorType } from './theme.tsx'
import { getColor } from './utils.ts'

/**
 * A card that can optionally be colored.
 *
 * **Examples**
 *
 * ```rust Padding::all(10).body(Card::new().body(Label::new("Hey! I am a card!"))) ```
 *
 * ```rust Padding::all(10).body(Card::new().body(Label::new("Hey! I am a red card!")).color(ColorType::Danger)) ```
 *
 * @component
 */
export interface Card {
	body?: Component
	color: ColorType
}

export function CardRender(props: Card) {
	const isSkeleton = useSkeletonDetection()

	if (isSkeleton) {
		return (
			<div class='relative w-full h-full p-2'>
				{props.body && <ComponentRender {...props.body} />}

				<div class='absolute inset-0'>
					<SkeletonBlock height={null} width={null} rounding='slight' />
				</div>
			</div>
		)
	}

	return (
		<div
			class={`
				w-full h-full border rounded-md
				border-${getColor({ kind: props.color, opacity: 10 })} bg-${getColor({ kind: props.color, opacity: 5 })}
			`}
		>
			{props.body && <ComponentRender {...props.body} />}
		</div>
	)
}
