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
 * **Example**
 *
 * ```rust Image::new("https://images.unsplash.com/photo-1711436470690-cf49602d1cf1?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D") .width(300) .height(300) .fit(ImageFit::Cover) .decorate() ```
 *
 * @component
 */
export interface Image {
	decorate: boolean
	fit: ImageFit
	height?: number
	position: ImagePosition
	url: string
	width?: number
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
