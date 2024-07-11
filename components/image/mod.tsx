import { cases, Image, ImageFit, ImagePosition, isOk, React } from 'runtime'
import { SkeletonBlock, useSkeletonDetection } from '../skeleton/mod.tsx'

const getFit = (fit: ImageFit) => {
	return `object-${cases.paramCase(fit)}`
}

const getPosition = (position: ImagePosition) => {
	return `object-${cases.paramCase(position)}`
}

export function ImageRender(props: Image) {
	const isSkeleton = useSkeletonDetection()

	const decorateClasses = 'border border-fore-10 rounded-md overflow-hidden'
	const extraSkeletonSize = props.decorate ? 2 : 0 // account for the borders

	return (
		<div class={`inline-block ${props.decorate && !isSkeleton ? decorateClasses : ''}`}>
			{isSkeleton
				? (
					<SkeletonBlock
						height={isOk(props.height) ? props.height + extraSkeletonSize : null}
						width={isOk(props.width) ? props.width + extraSkeletonSize : null}
						rounding={props.decorate ? 'slight' : 'non'}
					/>
				)
				: (
					<img
						class={`
						${props.width ? `w-${props.width}` : ''} ${props.height ? `h-${props.height}` : ''}
						${getFit(props.fit)} ${getPosition(props.position)}
					`}
						draggable={false}
						src={props.url}
					/>
				)}
		</div>
	)
}
