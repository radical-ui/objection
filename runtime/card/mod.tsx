import { Card, getColor, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'
import { SkeletonBlock, useSkeletonDetection } from '../skeleton/mod.tsx'

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
				border-${getColor({ type: props.color, opacity: 10 })} bg-${getColor({ type: props.color, opacity: 5 })}
			`}
		>
			{props.body && <ComponentRender {...props.body} />}
		</div>
	)
}
