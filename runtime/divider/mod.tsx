import { Divider, DividerDistinction, React } from 'runtime'

const getBreadth = (distinction: DividerDistinction) => {
	if (distinction === 'Profound') return 4
	if (distinction === 'Medium') return 2
	if (distinction === 'Slight') return 1
}

export function DividerRender(props: Divider) {
	const mainLetter = props.direction === 'Horizontal' ? 'w' : 'h'
	const crossLetter = props.direction === 'Vertical' ? 'w' : 'h'

	return <div class={`${mainLetter}-full ${crossLetter}-${getBreadth(props.distinction)} bg-fore-10`} />
}
