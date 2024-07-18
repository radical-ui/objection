import { Component, ComponentRender } from './component.tsx'
import { EventKey, getEventId, React, sendEvent } from './deps.ts'
import { useNoticeDispatch } from './notice.tsx'
import { ManyMap } from './utils.ts'

export type DispatchFn<T> = (payload: T) => Promise<void>

const dispatchStartListeners = new ManyMap<string, VoidFunction>()
const dispatchFinishListeners = new ManyMap<string, VoidFunction>()

const DisabledContext = React.createContext(false)

export interface ProvideDisabledContextProps {
	isDisabled: boolean
	children: React.ReactNode
}

export interface UseDispatcherResult<T> {
	isLoading: boolean
	isDisabled: boolean
	dispatch: DispatchFn<T>
}

export function useDispatcher<T>(key: EventKey<T> | null): UseDispatcherResult<T> {
	const isDisabled = React.useContext(DisabledContext)
	const dispatchNotice = useNoticeDispatch()

	const id = key === null ? null : getEventId(key)

	// NOTE from Elijah: I'm not a genius, but I think that this is borked. Don't know why it was working before.
	//
	// TODO(Elijah) fix this whole mess
	// Reasoning: nobody is ever going to fire two actions on the same `useDispatch` result at the same time, so why are counting?
	// I'm supposing this count was meant to be global, because it updates as the globals change. In that case, it should start off with
	// the current global value, not 0. It would be off it this component was mounted while an action was ongoing.
	//
	// Second thoughts: I suppose this never happened before due the the syncronous nature of http events/actions in the past, but this edge case
	// would be triggered a lot more with the onset of websocket connections and events/actions that are asyncronous and long-running.
	//
	// TODO food for thought. How is the server going to handle async events actions while preventing data races? Not a mutex please! Can you think
	// of anything better though??
	//    AMEND possible solution: there are generally two possible responses to events, those that can be responded to quickly (eg. username update), and long-running tasks
	//    (eg. do some super complex caluclation or data aggregation). The former will require full user context, meaning that they will need to be run
	//    syncronously. The latter can be spawned off on their own task and run with entirely owned data. These long t
	const [ongoingActionsCount, setOngoingActionsCount] = React.useState(0)

	React.useEffect(() => {
		if (id === null) return

		const unsubscribeStart = dispatchStartListeners.add(id, () => {
			setOngoingActionsCount((count) => count + 1)
		})

		const unsubscribeFinish = dispatchFinishListeners.add(id, () => {
			setOngoingActionsCount((count) => count - 1)
		})

		return () => {
			unsubscribeStart()
			unsubscribeFinish()
		}
	}, [id])

	const isLoading = ongoingActionsCount > 0
	const dispatch = React.useMemo(() => async (payload: T) => {
		if (key === null || id === null) return

		for (const listener of dispatchStartListeners.get(id)) listener()

		try {
			await sendEvent(key, payload)
		} catch (error) {
			console.error(error)

			if (dispatchNotice) dispatchNotice({ message: error.message || error.toString(), style: 'Error' })
		}

		for (const listener of dispatchFinishListeners.get(id)) listener()
	}, [id])

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
	const currentContext = React.useContext(DisabledContext)

	return (
		<DisabledContext.Provider value={props.block || currentContext}>
			<ComponentRender {...props.body} />
		</DisabledContext.Provider>
	)
}
