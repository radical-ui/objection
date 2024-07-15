import { colors, rollup, streamUtils } from './deps.ts'
import { flush, print } from './print.ts'

type Input = [string, { resolutions: Record<string, Record<string, string>>; source_files: Record<string, string> }]

const entryId = 'entry://default'

const unknownInput = await streamUtils.toJson(Deno.stdin.readable)
const [entryCode, { resolutions, source_files: sourceFiles }] = unknownInput as Input

const build = await rollup.rollup({
	input: entryId,
	plugins: [{
		name: 'loader',
		resolveId(source, currentModule) {
			if (source === entryId) return entryId
			if (currentModule === entryId) return source // all imports in the entry module are pre-resolved

			if (!currentModule) throw new Error('Because this was not the entry, expected there to be a current module')

			const resolved = resolutions[currentModule]?.[source]
			if (!resolved) {
				throw new Error(`Encountered a source "${source}" from module "${currentModule}", for which no resolution was provided`)
			}

			return resolved
		},
		async load(id) {
			if (id === entryId) return entryCode

			const sourceFile = sourceFiles[id]
			if (!sourceFile) throw new Error(`Encountered an id "${id}", for which no source file was provided`)

			return await Deno.readTextFile(sourceFile)
		},
		moduleParsed({ id }) {
			if (id === entryId) return

			print(`${colors.green('Bundle')} ${colors.gray(id)}`)
		},
		onLog(level, log) {
			if (level === 'warn') return false

			flush()
			console.error(`${colors.bold(`${colors.red('error')}:`)} ${log.message}`)

			return false
		},
	}],
})

print(colors.gray(`Generating...\r`))

const { output } = await build.generate({ sourcemap: 'inline', format: 'esm' })
const code = output[0].code

console.log(code)

flush()
