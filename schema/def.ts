export function defineElement<T extends ElementInfo>(element: T): T {
	return element
}

export type ElementInfo = {
	description: string
	params: Record<string, { description: string; required?: boolean } & Param>
}

export type Param = RecordParam | TextParam | EnumParam | SizeParam | ElementParam | ElementListParam | BooleanParam | NumberParam

export type BooleanParam = {
	type: 'boolean'
}

export type NumberParam = {
	type: 'number'
}

export type RecordParam = {
	type: 'record'
	items: Record<string, { description: string; required?: boolean } & Param>
}

export type TextParam = {
	type: 'text'
}

export type EnumParam = {
	type: 'enum'
	options: { id: string; description: string }[]
}

/**
 * Represents a size value. The expected values for this are a number (representing a pixel value),
 * or a string containing a static size, the options for which are defined in `staticSizes` */
export type SizeParam = {
	type: 'size'
}

export const staticSizes = ['xs', 'sm', 'md', 'lg', 'xl', '2xl', '3xl', '4xl', 'full']

export type ElementParam = {
	type: 'element'
	extend_params?: Record<string, Param & { description: string; required?: boolean }>
}

export type ElementListParam = {
	type: 'element_list'
	extend_params?: Record<string, Param & { description: string; required?: boolean }>
}
