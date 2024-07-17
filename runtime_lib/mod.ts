/**
 * An event that could be triggered, where `T` is the data that the event will contain
 *
 * @feature_event_key
 */
export type EventKey<T> = { path: string[]; debugSymbol?: string }

/**
 * An action that could be triggered, where `T` is the data that the action will contain
 *
 * @feature_action_key
 */
export type ActionKey<T> = { path: string[]; debugSymbol?: string }

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
		const listener = actionListeners.get(getJoinedKey(action.key))
		if (!listener) throw new Error(`No action listener was specified for action: ${JSON.stringify(action, null, '\t')}`)

		listener(action.data)
	}
}

export function registerActionListener<T>(key: ActionKey<T>, listener: (data: T) => void) {
	const joinedKey = getJoinedKey(key)

	// @ts-ignore at our best, we have to home that something isn't seriously borked up and trust that the key will always match the data
	actionListeners.set(joinedKey, listener)

	return () => {
		actionListeners.delete(joinedKey)
	}
}

function getJoinedKey(actionKey: ActionKey<unknown>) {
	return actionKey.path.map((item) => item.replaceAll(':', '\\:')).join('::')
}
