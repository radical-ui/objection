/**
 * An event that could be triggered, where `T` is the data that the event will contain
 *
 * @feature_event_key
 */
export type EventKey<T> = { event_path: string[]; debugSymbol?: string }

/**
 * An event that could be triggered, but that is not linked to any payload. This should only be used in cases where the event is not
 * actually triggered, but there is some reason to keep a reference to it.
 *
 * @feature_any_event
 */
export type AnyEvent = { event_path: string[]; debugSymbol?: string }

/**
 * An action that could be triggered, where `T` is the data that the action will contain
 *
 * @feature_action_key
 */
export type ActionKey<T> = { action_path: string[]; debugSymbol?: string }

const actionListeners = new Map<string, (d: unknown) => void>()
let sessionId: string | null = null
let endpoint: URL | null = null

export function setEndpoint(url: URL) {
	endpoint = url
}

export async function sendEvent<T>(key: EventKey<T>, data: T) {
	if (!sessionId) sessionId = crypto.randomUUID()
	if (!endpoint) throw new Error('Expected an endpoint to be set. Call `setEndpoint` before any calls to `sendEvent`')

	if (endpoint.protocol !== 'http:' && endpoint.protocol !== 'https:') {
		throw new Error(`'${endpoint.protocol}' endpoints are not supported`)
	}

	const response = await fetch(endpoint, {
		method: 'POST',
		body: JSON.stringify([{ key, data }]),
		headers: { 'content-type': 'application/json', 'x-session-id': sessionId },
	})
		.catch(() => null)

	if (!response) throw new Error('You appear to be offline')
	if (!response.ok) throw new Error(await response.text())

	const actions = await response.json() as { key: ActionKey<unknown>; data: unknown }[]

	for (const action of actions) {
		const listener = actionListeners.get(getActionId(action.key))
		if (!listener) throw new Error(`No action listener was specified for action: ${JSON.stringify(action, null, '\t')}`)

		listener(action.data)
	}
}

export function registerActionListener<T>(key: ActionKey<T>, listener: (data: T) => void) {
	const joinedKey = getActionId(key)

	// @ts-ignore at our best, we have to home that something isn't seriously borked up and trust that the key will always match the data
	actionListeners.set(joinedKey, listener)

	return () => {
		actionListeners.delete(joinedKey)
	}
}

export function getActionId(actionKey: ActionKey<unknown>) {
	return safeJoin(actionKey.action_path)
}

export function getEventId(eventKey: EventKey<unknown>) {
	return safeJoin(eventKey.event_path)
}

function safeJoin(path: string[]) {
	return path.map((item) => item.replaceAll(':', '\\:')).join('::')
}
