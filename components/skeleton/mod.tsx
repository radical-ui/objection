import { React, Skeleton, useDispatcher } from 'runtime'
import { ComponentRender } from '../mod.tsx'

const SkeletonContext = React.createContext(false)

export function SkeletonRender(props: Skeleton) {
	const { isLoading } = useDispatcher(props.linked_action)

	return (
		<SkeletonContext.Provider value={isLoading}>
			<ComponentRender {...props.body} />
		</SkeletonContext.Provider>
	)
}

export function useSkeletonDetection() {
	return React.useContext(SkeletonContext)
}

export type SkeletonRounding = 'slight' | 'full' | 'non'

export interface SkeletonBlockProps {
	width: number | null
	height: number | null
	rounding: SkeletonRounding
}

export function SkeletonBlock(props: SkeletonBlockProps) {
	const [darkness, setDarkness] = React.useState(0)
	const element = React.useRef<HTMLDivElement | null>(null)

	// smaller areas should be darker because they are harder to see
	const calculateDarkness = (width: number, height: number) => {
		const area = width * height
		const largestArea = 300 * 300
		const darkerTheLargerRatio = Math.min(area / largestArea, 1)
		const darkerTheSmallerRatio = Math.min(1 - darkerTheLargerRatio)
		const darkness = 2 + Math.round(darkerTheSmallerRatio * 10)

		return darkness
	}

	React.useEffect(() => {
		if (element.current) setDarkness(calculateDarkness(element.current.clientWidth, element.current.clientHeight))
	}, [element.current])

	// TODO use gradient and some point

	return (
		<div
			ref={element}
			class={`
				w-${props.width === null ? 'full' : Math.round(props.width)}
				h-${props.height === null ? 'full' : Math.round(props.height)}
				${props.rounding === 'full' ? 'rounded-full' : props.rounding === 'slight' ? 'rounded' : ''}
				animate-pulse bg-fore-${darkness}
			`}
		/>
	)
}
