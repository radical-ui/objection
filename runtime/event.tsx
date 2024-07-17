import { Component, ComponentRender } from './component.tsx'
import { React } from './deps.ts'
import { ManyMap } from './utils.ts'

export type EventKey = string

export interface Event {
	key: EventKey
	payload?: unknown
}

export type RawDispatchFn = (eventTree: Event[]) => Promise<void>
export type DispatchFn = (payload: unknown) => Promise<void>

const dispatchStartListeners = new ManyMap<string, VoidFunction>()
const dispatchFinishListeners = new ManyMap<string, VoidFunction>()

const DispatchContext = React.createContext<RawDispatchFn>(() => {
	console.warn('Expected an event dispatcher to be set')

	return Promise.resolve()
})

const ScopeContext = React.createContext<Event[]>([])
const DisabledContext = React.createContext(false)

export interface ProvideDispatchProps {
	dispatch: RawDispatchFn
	children: React.ReactNode
}

export function ProvideDispatch(props: ProvideDispatchProps) {
	return <DispatchContext.Provider value={props.dispatch}>{props.children}</DispatchContext.Provider>
}

export interface ProvideScopeProps {
	scope: Event[]
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

export function useEventScope() {
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
	const scope = useEventScope()
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
export interface EventBlocker {
	block: boolean
	body?: Component
}

export function EventBlockerRender(props: EventBlocker) {
	return (
		<ProvideDisabledContext isDisabled={props.block}>
			{props.body && <ComponentRender {...props.body} />}
		</ProvideDisabledContext>
	)
}

/**
 * A container that prefixes all events triggered within with `scope`
 *
 * **Example**
 *
 * ```rust #[derive(HasActionKey, Serialize, Deserialize)] enum Event { Foo, Bar, }
 *
 * ActionScope::new(Event::Foo).payload(serde_json::json!({ "here": true })).body(Button::new("Click me").event(Event::Bar)) ```
 *
 * @component
 */
export interface EventScope {
	body?: Component
	payload?: unknown
	scope: EventKey
}

export function EventScopeRender(props: EventScope) {
	const parentScope = useEventScope()

	return (
		<div class='w-full h-full'>
			<ProvideScope scope={[{ key: props.scope, payload: props.payload ?? null }, ...parentScope]}>
				{props.body ? <ComponentRender {...props.body} /> : <></>}
			</ProvideScope>
		</div>
	)
}
