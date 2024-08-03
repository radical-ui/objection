import { useDispatcher } from './event.tsx'
import { ComponentRender } from './component.tsx'
import { Component, React } from './deps.ts'

const SkeletonContext = React.createContext(false)

/**
 * TODO
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo }
 *
 * Padding::all(10) .body( Flex::new(FlexKind::Column) .gap(10) .auto_item( Skeleton::new( Event::Foo, Card::new().body( Flex::new(FlexKind::Column) .gap(10) .auto_item( RadioInput::new() .item(0, "Hi") .described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend")) .described_item(2, "Adieu", Label::new("The french form of \"Bye\"")) ) .auto_item( Image::new("https://images.unsplash.com/photo-1716369415085-4a6876f91840?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxlZGl0b3JpYWwtZmVlZHwyfHx8ZW58MHx8fHx8") .width(300) .height(200) .fit(ImageFit::Cover) .decorate() ) ) ) ) .auto_item(Button::new("Load").event(Event::Foo)) ) ```
 *
 * @component
 */
export interface Skeleton {
	linkedEventSymbols?: string[]
	body?: Component
}

export function SkeletonRender(props: Skeleton) {
	const { isLoading } = useDispatcher(props.linkedEventSymbols ? { eventPath: props.linkedEventSymbols } : null)

	return (
		<SkeletonContext.Provider value={isLoading}>
			{props.body && <ComponentRender {...props.body} />}
		</SkeletonContext.Provider>
	)
}

export function useSkeletonDetection() {
	return React.useContext(SkeletonContext)
}

export type SkeletonRounding = 'slight' | 'full' | 'none'

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
