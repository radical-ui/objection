import { program } from 'commander'
import { elements } from '~/schema'
import { generateTypes } from './gen_typescript'
import { buildAndWriteAll } from './build'

program
	.command('build')
	.option('-o, --out', 'The location to write the resulting binary to')
	.action(async options => {
		const out: string = options.out ?? './dist'

		await buildAndWriteAll({
			outDir: out,
			sdkPath: '.',
		})
	})

program.command('generate <kind>').action(kind => {
	if (kind === 'json') {
		console.log(JSON.stringify(elements))
		return
	}

	if (kind === 'ts' || kind === 'typescript') {
		console.log(generateTypes())
		return
	}

	throw new Error('Invalid generate kind. Expected `ts` or `json`.')
})

program.parse()
