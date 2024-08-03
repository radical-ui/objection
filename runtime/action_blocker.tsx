import { ComponentRender } from './component.tsx'
import { Component, React } from './deps.ts'
import { ManyMap } from './utils.ts'

export type ActionKey = string

export interface Action {
	key: ActionKey
	payload?: unknown
}

export type RawDispatchFn = (eventTree: Action[]) => Promise<void>
export type DispatchFn = (payload: unknown) => Promise<void>

const dispatchStartListeners = new ManyMap<string, VoidFunction>()
const dispatchFinishListeners = new ManyMap<string, VoidFunction>()

const DispatchContext = React.createContext<RawDispatchFn>(() => {
	console.warn('Expected an event dispatcher to be set')

	return Promise.resolve()
})

const ScopeContext = React.createContext<Action[]>([])
const DisabledContext = React.createContext(false)

export interface ProvideDispatchProps {
	dispatch: RawDispatchFn
	children: React.ReactNode
}

export function ProvideDispatch(props: ProvideDispatchProps) {
	return <DispatchContext.Provider value={props.dispatch}>{props.children}</DispatchContext.Provider>
}

export interface ProvideScopeProps {
	scope: Action[]
	children: React.ReactNode
}

export function ProvideScope(props: ProvideScopeProps) {
	return <ScopeContext.Provider value={props.scope}>{props.children}</ScopeContext.Provider>
}

export interface ProvideDisabledContextProps {
	isDisabled: boolean
	children: React.ReactNode
}

export function ProvideDisabledContext(props: ProvideDisabledContextProps) {
	return <DisabledContext.Provider value={props.isDisabled}>{props.children}</DisabledContext.Provider>
}

export function useRawDispatch() {
	return React.useContext(DispatchContext)
}

export function useActionScope() {
	return React.useContext(ScopeContext)
}

export function useDisabledContext() {
	return React.useContext(DisabledContext)
}

export interface UseDispatcherResult {
	isLoading: boolean
	isDisabled: boolean
	dispatch: DispatchFn
}

export function useDispatcher(id: string | null): UseDispatcherResult {
	const rawDispatch = useRawDispatch()
	const scope = useActionScope()
	const isDisabled = useDisabledContext()

	const fullId = id === null ? null : `${scope.map((event) => event.key).join(',')},${id}`
	const [ongoingActionsCount, setOngoingActionsCount] = React.useState(0)

	React.useEffect(() => {
		if (fullId === null) return

		const unsubscribeStart = dispatchStartListeners.add(fullId, () => {
			setOngoingActionsCount((count) => count + 1)
		})

		const unsubscribeFinish = dispatchFinishListeners.add(fullId, () => {
			setOngoingActionsCount((count) => count - 1)
		})

		return () => {
			unsubscribeStart()
			unsubscribeFinish()
		}
	}, [id])

	const isLoading = ongoingActionsCount > 0
	const dispatch = React.useMemo(() => async (payload: unknown) => {
		if (fullId === null) return

		for (const listener of dispatchStartListeners.get(fullId)) listener()
		await rawDispatch([{ key: id!, payload }, ...scope])
		for (const listener of dispatchFinishListeners.get(fullId)) listener()
	}, [id, scope])

	return { isLoading, dispatch, isDisabled: isDisabled || id === null }
}

/**
 * TODO
 *
 * **Example**
 *
 * ```rust #[derive(Debug, HasActionKey, Serialize, Deserialize)] pub enum Event { Foo }
 *
 * ActionBlocker::new().body(Button::new("Disabled").event(Event::Foo)) ```
 *
 * @component
 */
export interface ActionBlocker {
	block: boolean
	body?: Component
}

export function ActionBlockerRender(props: ActionBlocker) {
	return (
		<ProvideDisabledContext isDisabled={props.block}>
			{props.body && <ComponentRender {...props.body} />}
		</ProvideDisabledContext>
	)
}
