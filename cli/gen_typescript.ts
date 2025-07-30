import { elements, staticSizes, type ElementInfo, type ElementListParam, type ElementParam, type Param } from '~/schema'
import { pascal } from 'case'

const indent = (level: number): string => '\t'.repeat(level)

const generateElementType = (param: ElementParam | ElementListParam, level: number): string => {
	if (!param.extend_params) return 'Element'

	const extendedFields = Object.entries(param.extend_params)
		.map(([childId, childParam]) => {
			const childLevel = level + 1
			const docComment = generateDocComment(childParam.description, childLevel)

			return `${docComment}\n${indent(childLevel)}${childId}: ${generateParamType(childParam, childLevel)}`
		})
		.join('\n')

	return `(Element & {\n${extendedFields}\n${indent(level)}})`
}

const generateParamType = (param: Param, level: number): string => {
	if (param.type === 'text') return 'string'
	if (param.type === 'boolean') return 'boolean'
	if (param.type === 'number') return 'number'
	if (param.type === 'size') return 'Size'
	if (param.type === 'enum') return param.options.map(opt => `'${opt.id}'`).join(' | ')
	if (param.type === 'element') return generateElementType(param, level)
	if (param.type === 'element_list') return `${generateElementType(param, level)}[]`
	if (param.type === 'record') {
		const fields = Object.entries(param.items)
			.map(([key, value]) => {
				const docComment = value.description ? `${indent(level + 1)}/** ${value.description} */\n` : ''
				const requiredness = value.required ? '' : '?'
				return `${docComment}${indent(level + 1)}${key}${requiredness}: ${generateParamType(value, level + 1)}`
			})
			.join('\n')
		return `{\n${fields}\n${indent(level)}}`
	}

	throw new Error(`Invalid param type`)
}

const generateDocComment = (description: string, level: number) => {
	return `${indent(level)}/** ${description.split('\n').join(`\n${indent(level)} * `)} */`
}

const generateTypesForElement = (name: string, info: ElementInfo, level: number): string => {
	const typeName = pascal(name)
	const elementDoc = generateDocComment(info.description, level)

	const params = Object.entries(info.params)
		.map(([key, param]) => {
			const paramDoc = generateDocComment(param.description, level + 1)
			const requiredness = param.required ? '' : '?'
			return `${paramDoc}\n${indent(level + 1)}${key}${requiredness}: ${generateParamType(param, level + 1)}`
		})
		.join('\n\n')

	return `${elementDoc}\n${indent(level)}export type ${typeName} = {\n${indent(level + 1)}$: '${name}'\n\n${params}\n${indent(level)}}`
}

const generateBuilderClass = (name: string, info: ElementInfo, level: number): string => {
	const typeName = pascal(name)
	const builderName = `${typeName}Builder`
	const elementDoc = generateDocComment(info.description, level)

	const methods = Object.entries(info.params)
		.filter(([key]) => key !== '$') // Skip the $ property as it's set in constructor
		.map(([key, param]) => {
			const description = `${param.description}${param.type === 'enum' ? '\n * ' + param.options.map(opt => `\`${opt.id}\``).join(' | ') : ''}`
			const methodDoc = `${indent(level + 1)}/** ${description} */`
			const paramType = generateParamType(param, level + 1)
			return `${methodDoc}\n${indent(level + 1)}${key}(${key}: ${paramType}) {\n${indent(level + 2)}this.state.${key} = ${key}\n${indent(level + 2)}return this\n${indent(level + 1)}}`
		})
		.join('\n\n')

	return `${elementDoc}\nexport class ${builderName} {\n${indent(level + 1)}state: ${typeName}\n\n${indent(level + 1)}constructor(state: ${typeName}) {\n${indent(level + 2)}this.state = state\n${indent(level + 1)}}\n\n${methods}\n${indent(level)}}`
}

const generateFactoryFunction = (name: string, info: ElementInfo, level: number): string => {
	const typeName = pascal(name)

	const requiredParams = Object.entries(info.params).filter(([_, param]) => param.required)
	const funcArgs = requiredParams.map(([key, param]) => `${key}: ${generateParamType(param, level)}`).join(', ')
	const suppliedBuilderParams = requiredParams.map(([key, _]) => key).join(', ')
	const builderParams = requiredParams.length ? `{ $: '${name}', ${suppliedBuilderParams} }` : `{ $: '${name}' }`

	const contents = `return new ${typeName}Builder(${builderParams})`
	const funcSignature = `export function ${name}(${funcArgs})`

	return `${indent(level)}${funcSignature} {\n${indent(level + 1)}${contents}\n${indent(level)}}`
}

const generateSizeType = () => {
	const comment = `/** Accepts pixels (number) or preset sizes */`
	const staticTs = staticSizes.map(item => `'${item}'`).join(' | ')
	return `${comment}\nexport type Size = number | ${staticTs}`
}

const generateGetChildElements = (): string => {
	const cases = Object.entries(elements)
		.map(([name, info]) => {
			// Find all parameters that are of type 'element' or 'element_list'
			const childParams = Object.entries(info.params)
				.filter(([_, param]) => param.type === 'element' || param.type === 'element_list')
				.map(([paramName, param]) => {
					if (param.type === 'element') {
						return `if (element.${paramName}) children.push(element.${paramName})`
					} else if (param.type === 'element_list') {
						return `if (element.${paramName}) children.push(...element.${paramName})`
					}
					return ''
				})
				.filter(Boolean)

			if (childParams.length === 0) {
				return `${indent(2)}case '${name}':\n${indent(3)}break`
			}

			const childLogic = childParams.join(`\n${indent(3)}`)
			return `${indent(2)}case '${name}':\n${indent(3)}${childLogic}\n${indent(3)}break`
		})
		.join('\n')

	return `/** Gets all child elements for a given element */
export function getChildElements(element: Element): Element[] {
${indent(1)}const children: Element[] = []
${indent(1)}
${indent(1)}switch (element.$) {
${cases}
${indent(1)}}
${indent(1)}
${indent(1)}return children
}`
}

export function generateTypes(): string {
	const unionType = Object.keys(elements)
		.map(name => pascal(name))
		.join(' | ')

	const elementTypes = Object.entries(elements)
		.map(([name, info]) => generateTypesForElement(name, info, 0))
		.join('\n\n')

	const builderClasses = Object.entries(elements)
		.map(([name, info]) => generateBuilderClass(name, info, 0))
		.join('\n\n')

	const factoryFunctions = Object.entries(elements)
		.map(([name, info]) => generateFactoryFunction(name, info, 0))
		.join('\n\n')

	const header = '// This file is auto-generated. Do not edit directly.'
	const element = `export type Element = ${unionType}\n\n${elementTypes}`

	const getChildElementsFunction = generateGetChildElements()

	return `${header}\n\n${generateSizeType()}\n\n${element}\n\n${builderClasses}\n\n${factoryFunctions}\n\n${getChildElementsFunction}`
}
