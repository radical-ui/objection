import { file, type BunPlugin } from 'bun'
import { readdir } from 'fs/promises'
import pathUtils from 'path'

const resolveElement = async (sdkPath: string, element: string) => {
	const path = pathUtils.join(sdkPath, 'elements', `${element}.svelte`)
	if (await file(path).exists()) return path

	throw new Error(`element '${element}' not found`)
}

type ResolvedElement = {
	name: string
	path: string
}

const resolveElements = async (sdkPath: string, elements: string[]): Promise<ResolvedElement[]> => {
	const paths: ResolvedElement[] = []

	for (const element of elements) {
		paths.push({
			name: element,
			path: await resolveElement(sdkPath, element),
		})
	}

	return paths
}

const getAllElements = async (sdkPath: string): Promise<ResolvedElement[]> => {
	const suffix = '.svelte'
	const paths: ResolvedElement[] = []

	for (const path of await readdir(pathUtils.join(sdkPath, 'elements'))) {
		if (!path.endsWith(suffix) || path.startsWith('_')) continue

		paths.push({
			name: path.slice(0, path.length - suffix.length),
			path: pathUtils.join(sdkPath, 'elements', path),
		})
	}

	return paths
}

const generateElementPicker = (elements: ResolvedElement[]) => {
	const imports = elements.map(({ name, path }) => `import ${name} from '${path}'`).join('\n')
	const statements = elements.map(({ name }) => `if (name === '${name}') return ${name}`).join('\n')
	const invalidStatement = `throw new Error("element '\${name}' not found")`
	const defaultExport = `export default (name) => {\n\t${statements}\n\t${invalidStatement}\n}`

	return `${imports}\n\n${defaultExport}`
}

// TODO get rid of `specificElements` in favor of fragments, which will need to be a different plugin
export async function elementPickerPlugin(sdkPath: string, specificElements: string[] | null): Promise<BunPlugin> {
	const elements = specificElements ? await resolveElements(sdkPath, specificElements) : await getAllElements(sdkPath)
	const code = generateElementPicker(elements)

	return {
		name: 'element picker',
		setup(builder) {
			builder
				.onResolve({ filter: /^@internal\/element_picker$/ }, () => ({ path: 'virtual', namespace: 'element_picker' }))
				.onLoad({ namespace: 'element_picker', filter: /.*/ }, () => ({
					contents: code,
					loader: 'js',
				}))
		},
	}
}
