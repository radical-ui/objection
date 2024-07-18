// We're going to try to do this without a context and provider because we already have enough of those, let's see how it goes!

import { React } from './deps.ts'

const titleSegmentsOrder: number[] = []
const titleSegmentsData = new Map<number, string>()

let idSequence = 0

/**
 * A component for setting the a title segment. Will be joined to any title segments in the component hierarchy using a '|'
 *
 * @component
 */
export interface TitleSegment {
	title: string
}

export function TitleSegmentRender(props: TitleSegment) {
	const [currentId] = React.useState(() => idSequence++)

	if (!titleSegmentsData.has(currentId)) {
		titleSegmentsOrder.push(currentId)
	}

	React.useEffect(() => {
		titleSegmentsData.set(currentId, props.title)
		flushTitle()

		return () => {
			titleSegmentsData.delete(currentId)
			flushTitle()
		}
	}, [props.title])

	React.useEffect(() => {
		return () => titleSegmentsOrder.splice(titleSegmentsOrder.indexOf(currentId))
	}, [])

	return <></>
}

function flushTitle() {
	document.title = titleSegmentsOrder.map((id) => titleSegmentsData.get(id) ?? '').filter((item) => item.length).join(' | ')
}
