import { cases, denoDoc, dtils, Fuzzy, pathUtils } from './deps.ts'
import { errorLog, successLog, warnLog } from './utils.ts'

export interface Component {
	name: string
	tsPath: string
	tsRenderName: string
	rsPath: string
	rsExamples: string[]
}

export async function collect() {
	const components: Component[] = []
	const componentNames: string[] = []

	for await (const entry of Deno.readDir('components')) {
		const rawName = entry.name

		if (rawName === 'mod.rs' || rawName === 'mod.tsx' || rawName === 'readme.md') continue

		if (!entry.isDirectory) {
			warnLog('All components must be directories')
			continue
		}

		componentNames.push(rawName)
	}

	const createWorker = async () => {
		const rawName = componentNames.pop()
		if (!rawName) return

		const component = await loadComponent(
			rawName,
			pathUtils.join('components', rawName, 'mod.rs'),
			pathUtils.join('components', rawName, 'mod.tsx'),
		)

		if (component) {
			components.push(component)
			successLog('Load', rawName)
		}

		await createWorker()
	}

	const runWorkers = async (amountParallel: number) => {
		const promises: Promise<void>[] = []

		for (let i = 0; i < amountParallel; i++) promises.push(createWorker())

		return await Promise.all(promises)
	}

	await runWorkers(20)

	return components.sort((a, b) => a.name.localeCompare(b.name))
}

async function loadComponent(name: string, rsPath: string, tsPath: string): Promise<Component | null> {
	if (!await dtils.exists(tsPath)) {
		errorLog(`Expected to find a TSX file at ${tsPath}`)
		return null
	}

	const tsRenderName = await getRenderExport(tsPath, name)
	if (!tsRenderName) {
		errorLog(`Expected to find a render function exported from ${tsPath}`)
		return null
	}

	if (!await dtils.exists(rsPath)) {
		errorLog(`Expected to find a rust file at ${rsPath}`)
		return null
	}

	const rsExamples = getRustExamples(await dtils.readText(rsPath))
	if (!rsExamples.length) warnLog(`Add usage examples to ${rsPath}`)

	return { name, rsExamples, rsPath, tsPath, tsRenderName }
}

async function getRenderExport(file: string, name: string) {
	const base = `file://${Deno.cwd()}/`
	const nodes = await denoDoc.doc(new URL(file, base).toString(), { importMap: base + 'import_map.json' })
	const exports: string[] = []

	for (const node of nodes) {
		if (node.kind === 'function') exports.push(node.name)
	}

	return tryMatch(exports, `${cases.pascalCase(name)}Render`) || exports[0] || null
}

function getRustExamples(code: string) {
	const regex = /```(rust|rs)([^`]*)```/g
	const examples: string[] = []

	for (const match of code.matchAll(regex)) {
		const text = match[2]
		const lines = text.split('\n').map((line) => line.startsWith('///') ? line.slice(3).trim() : line)

		examples.push(lines.join('\n'))
	}

	return examples
}

function tryMatch(haystack: string[], needle: string): string | null {
	const results = new Fuzzy().search(haystack, needle)[0]
	if (!results) return null

	const matchedIndex = results[0]
	return haystack[matchedIndex]
}
