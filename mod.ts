import { Color } from './lib/color.ts'
import { dtils, streams } from './lib/deps.ts'

const configValues = new dtils.SafeUnknown(await streams.toJson(Deno.stdin.readable)).asObject()

const getValue = (id: string, expectedType: string) => {
	const object = configValues.get(id).asObject()
	const type = object.get('type').asString()
	if (type !== expectedType) throw new Error(`${id} is a ${type}, but expected it to be a ${expectedType}`)

	return object.get('value')
}

export function getString(id: string) {
	return getValue(id, 'string').asString()
}

export function getColor(id: string) {
	const object = getValue(id, 'color').asObject()

	return new Color(object.get('red').asNumber(), object.get('green').asNumber(), object.get('blue').asNumber())
}

export function getNumber(id: string) {
	return getValue(id, 'number').asNumber()
}

export function getImage(id: string) {
	return getValue(id, 'image').asString()
}

export { Color }
