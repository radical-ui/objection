import { cases, React } from './deps.ts'
import { SkeletonBlock, useSkeletonDetection } from './skeleton.tsx'
import { isOk } from './utils.ts'

const getFit = (fit: ImageFit) => {
	return `object-${cases.paramCase(fit)}`
}

const getPosition = (position: ImagePosition) => {
	return `object-${cases.paramCase(position)}`
}

export type ImageFit = 'Contain' | 'Cover' | 'Fill' | 'None' | 'ScaleDown'
export type ImagePosition =
	| 'Bottom'
	| 'Center'
	| 'Left'
	| 'LeftBottom'
	| 'LeftTop'
	| 'Right'
	| 'RightBottom'
	| 'RightTop'
	| 'Top'

/**
 * TODO
 *
 * @component
 */
export interface Image {
	url: string

	decorate?: boolean
	fit?: ImageFit
	height?: number
	position?: ImagePosition
	width?: number
}

export function ImageRender(props: Image) {
	const fit = props.fit || 'Cover'
	const position = props.position || 'Center'

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
						rounding={props.decorate ? 'slight' : 'none'}
					/>
				)
				: (
					<img
						class={`
						${props.width ? `w-${props.width}` : ''} ${props.height ? `h-${props.height}` : ''}
						${getFit(fit)} ${getPosition(position)}
					`}
						draggable={false}
						src={props.url}
					/>
				)}
		</div>
	)
}
