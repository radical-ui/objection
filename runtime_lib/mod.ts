/**
 * A component
 *
 * @feature_component_index
 */
export interface Component {
	aUniqueKeyToHelpPreventThisTypeFromBeingAny: number
}

export function getComponentRenderer<Params, Return>(component: Component): { func: () => Return; params: Params } {
	return getNamespace().selectComponentRenderer(component)
}

/**
 * An event that could be triggered, where `T` is the data that the event will contain
 *
 * @feature_event_key
 */
export type EventKey<T> = { eventPath: string[]; debugSymbol?: string }

/**
 * An action that could be triggered, where `T` is the data that the action will contain
 *
 * @feature_action_key
 */
export type ActionKey<T> = { actionPath: string[]; debugSymbol?: string }

/**
 * An event that could be triggered, but that is not linked to any payload. This should only be used in cases where the event is not
 * actually triggered, but there is some reason to keep a reference to it.
 */
export interface AnyEvent {
	actionPath: string[]
	debugSymbol?: string
}

export const MOUNT_ACTION: ActionKey<Component> = {
	actionPath: ['root_mount'],
	debugSymbol: 'Mount at root',
}

export const READY_EVENT: EventKey<{ token: string | null }> = {
	eventPath: ['root_app_ready'],
	debugSymbol: 'Application is ready to be mounted',
}

export async function sendEvent<T>(key: EventKey<T>, data: T) {
	return await getNamespace().sendEvent(key, data)
}

export function registerActionListener<T>(key: ActionKey<T>, listener: (data: T) => void) {
	return getNamespace().registerActionListener(key, listener)
}

export function getEventId<T>(event: EventKey<T>): string {
	return getNamespace().getEventId(event)
}

function getNamespace() {
	// @ts-ignore no real good way to test this. Additionally, runtime_lib should be going away soon, so we won't have to do this TS/JS integration anymore
	const namespace = globalThis.window.OBJECTION
	if (!namespace || typeof namespace !== 'object') {
		throw new Error(
			"Could not find the necessary objection runtime setup. Perhaps you didn't build this project with the objection CLI?",
		)
	}

	return namespace
}
