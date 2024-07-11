import { Notice, React } from 'runtime'

interface Context {
	stashedNotices: Notice[]
	clear(index: number): void
}

const Context = React.createContext<Context | null>(null)

/** Returns the first available notice */
export function useNotices(): [Notice | null, (() => void)] {
	const context = React.useContext(Context)
	if (!context) throw new Error('Expected to find ProvideNotices in a root')

	const { clear, stashedNotices } = context

	if (!stashedNotices[0]) return [null, () => {}]

	return [stashedNotices[0], () => clear(0)]
}

export interface ProvideNoticeProps {
	children?: React.ReactNode
	setNoticeListener(fn: (notice: Notice) => void): void
}

export function ProvideNotices(props: ProvideNoticeProps) {
	const [stashedNotices, setStashedNotices] = React.useState<Notice[]>([])

	React.useEffect(() => {
		props.setNoticeListener((notice) => {
			setStashedNotices([...stashedNotices, notice])
		})
	}, [])

	return (
		<Context.Provider
			value={{
				stashedNotices,
				clear(index) {
					setStashedNotices([...stashedNotices.slice(0, index), ...stashedNotices.slice(index + 1)])
				},
			}}
		>
			{props.children}
		</Context.Provider>
	)
}
