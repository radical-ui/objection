import { React } from './deps.ts'
import { isOk } from './utils.ts'

export type UpdateFn = (update: unknown) => void

export class UpdateManager {
	#listeners = new Map<number, UpdateFn>()

	listen(id: number, fn: UpdateFn) {
		this.#listeners.set(id, fn)

		return () => {
			this.#listeners.delete(id)
		}
	}

	update(id: number, data: unknown) {
		const listener = this.#listeners.get(id)
		if (!listener) return console.error(`No update boundary was listening for surgical update ${id}`)

		listener(data)
	}
}

const Context = React.createContext<UpdateManager | null>(null)

export function useUpdates<T>(id: number | null): T | null {
	const manager = React.useContext(Context)
	if (!manager) throw new Error('Expected to find a <ProvideUpdateManager> element above any calls to the useUpdates hook')

	const [update, setUpdate] = React.useState<T | null>(null)

	React.useEffect(() => {
		if (!isOk(id)) return

		return manager.listen(id, (update) => setUpdate(update as T))
	}, [id])

	return update
}

export interface ProvideSurgicalUpdateManagerProps {
	manger: UpdateManager
	children: React.ReactNode
}

export function ProvideUpdateManager(props: ProvideSurgicalUpdateManagerProps) {
	return <Context.Provider value={props.manger}>{props.children}</Context.Provider>
}
