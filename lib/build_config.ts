import { Color } from './color.ts'
import { Image } from './image.ts'
import { dtils } from './deps.ts'

export type ImageKind = 'png' | 'jpg'

export type ParamType =
	| { $: 'string' }
	| { $: 'color' }
	| { '$': 'number' }
	| { '$': 'image'; kinds: ImageKind[] }

export function parseString(value: dtils.SafeUnknown, buildConfig: BuildConfig) {
}

export interface Param {
	name: string
	description: string
	type: ParamType
	default: unknown | null
	optional: boolean
	deploy_only: boolean
	preview_only: boolean
}

export interface Script {
	executor: 'deno'
	file: string
}

export interface BuildConfig {
	params: Record<string, Param>
	preview_script: Script
	deploy_script: Script
}

export function deserializeBuildConfig(raw: unknown): BuildConfig {
	const unknown = new dtils.SafeUnknown(raw)
	const object = unknown.asObject()

	return {
		params: object.get('params').asObject().map((unknown): Param => {
			const object = unknown.asObject()
			const type = getType(object.get('type'))

			return {
				name: object.get('name').asString(),
				description: object.get('description').asString(),
				type,
				default: ifNullOr(object.get('default'), null, (unknown) => getTypeValue(type, unknown)),
				optional: ifNullOr(object.get('optional'), false, (unknown) => unknown.asBoolean()),
				deploy_only: ifNullOr(object.get('optional'), false, (unknown) => unknown.asBoolean()),
				preview_only: ifNullOr(object.get('optional'), false, (unknown) => unknown.asBoolean()),
			}
		}),
		preview_script: getScript(object.get('preview_script')),
		deploy_script: getScript(object.get('deploy_script')),
	}
}

export function getString(type: ParamType, unknown: dtils.SafeUnknown) {
	if (type.$ !== 'string') throw new Error('Called getString on a type param that is not a string')

	return unknown.asString()
}

export function getNumber(type: ParamType, unknown: dtils.SafeUnknown) {
	if (type.$ !== 'number') throw new Error('Called ')

	return unknown.asNumber()
}

export function getImage(unknown: dtils.SafeUnknown) {
	return new Image(unknown.asString())
}

export function getColor(unknown: dtils.SafeUnknown) {
	const object = unknown.asObject()

	return new Color(object.get('red').asNumber(), object.get('green').asNumber(), object.get('blue').asNumber())
}

function ifNullOr<N, T>(value: dtils.SafeUnknown, ifNull: N, or: (u: dtils.SafeUnknown) => T): N | T {
	if (value.isNull()) return ifNull
	return or(value)
}

function getTypeValue(type: ParamType, value: dtils.SafeUnknown): unknown {
	if (type.$ === 'string') return value
	if (type.$ === 'number') return getString(value)
	if (type.$ === 'color') return getColor(value)
	if (type.$ === 'image') return getImage(value)
}

function getType(unknown: dtils.SafeUnknown): ParamType {
	const object = unknown.asObject()
	const descriminator = object.get('$').asString()

	if (descriminator === 'string') return { $: 'string' }
	if (descriminator === 'color') return { $: 'color' }
	if (descriminator === 'number') return { $: 'number' }
	if (descriminator === 'image') {
		const kinds = object.get('kinds').asArray().map((unknown): ImageKind => {
			const kind = unknown.asString()

			if (kind !== 'png' && kind !== 'jpg') throw new Error('Only "png" and "jpg" kinds are supported')

			return kind
		})

		return { $: 'image', kinds }
	}

	throw new Error(`Invalid type: "${descriminator}"`)
}

function getScript(unknown: dtils.SafeUnknown): Script {
	const object = unknown.asObject()
	const executor = object.get('executor').asString()

	if (executor !== 'deno') throw new Error("The only valid executor is 'deno'")

	return {
		executor: 'deno',
		file: object.get('file').asString(),
	}
}
