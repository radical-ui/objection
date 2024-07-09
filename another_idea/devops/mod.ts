import { collect } from './collect.ts'
import { cases, dtils, frontier, httpUtils, pathUtils } from './deps.ts'
import { generateRustExamples, generateRustMod, generateTsMod, generateTypes } from './generate.ts'
import { fetchIcons } from './icons.ts'
import { formatRs, formatTs, perhapsWrite, successLog } from './utils.ts'

export { fetchIcons } from './icons.ts'

export async function dev() {
	await gen()

	await dtils.sh('cargo build --package svelte_toolbox_expose_dev --bin get_examples')
	const { logLines } = await dtils.shCapture('./target/debug/get_examples')

	await frontier.startPreviewServer({
		entry: new URL('./dev_main.ts', import.meta.url),
		reload: true,
		async template(mainHead) {
			const html = await dtils.readText('devops/dev.html')

			return html.replace('{main_head}', mainHead).replace('{root_ui}', logLines.join('\n'))
		},
	})
}

export async function check() {
	await dtils.sh('deno check devops/mod.ts runtime/main.ts')
	await dtils.sh('cargo check')
	await dtils.sh('cargo check --package svelte_toolbox_derive_action_key')
	await dtils.sh('cargo check --package svelte_toolbox_expose_dev')
}

export async function gen() {
	const components = await collect()
	await Promise.all([
		generateRustMod(components, 'components/mod.rs'),
		generateRustExamples(components),
		generateTsMod(components, 'components/mod.tsx'),
	])

	await generateTypes()
}

export async function add(args: string[]) {
	const [name] = args
	if (!name) throw new Error('Expected a name as the first argument')

	const fileBase = pathUtils.join('components', cases.snakeCase(name), 'mod')
	const type = cases.pascalCase(name)

	await perhapsWrite(
		`${fileBase}.rs`,
		await formatRs(`
			use schemars::JsonSchema;
			use serde::{Deserialize, Serialize};

			/// TODO
			///
			/// **Example**
			///
			/// \`\`\`rust
			/// ${type}::new()
			/// \`\`\`
			#[derive(Debug, Serialize, Deserialize, JsonSchema)]
			pub struct ${type} {
			}

			impl ${type} {
				pub fn new() -> ${type} {
					${type} {}
				}
			}
		`),
	)

	await perhapsWrite(
		`${fileBase}.tsx`,
		await formatTs(`
			import { React, ${type} } from 'runtime'

			export function ${type}Render(props: ${type}) {
				return (
					<div>TODO</div>
				)
			}
		`),
	)

	await gen()
}

export async function build() {
	const entry = new URL('../runtime/main.ts', import.meta.url)
	successLog('Bundle', entry.toString())

	const { code, map } = await frontier.bundle(entry, { minify: true })

	await dtils.writeText(`target/build.js.map`, map)
	await dtils.writeText(`www/build.js`, code)

	await fetchIcons()
	await gen()
}

export async function preview(args: string[]) {
	const example = args[0]

	await build()

	const server = Deno.serve({ port: 8000 }, (request) => httpUtils.serveDir(request, { fsRoot: 'www' }))
	const promises = [server.finished]

	if (example) {
		promises.push(
			Promise.resolve().then(async () => {
				if (!await dtils.exists(`examples/${example}`) && !await dtils.exists(`examples/${example}.rs`)) {
					throw new Error(`Example "${example}" was selected, but it doesn't exist`)
				}
				successLog('Selected', `examples/${example}`)

				return dtils.sh(`cargo run --example ${example}`, {
					env: { SVELTE_TOOLBOX_URL: 'http://localhost:8000', RUST_LOG: Deno.env.get('RUST_LOG') ?? 'debug', SESSION_TIMEOUT: `${60 * 15}` },
				})
			}).catch(() => server.shutdown()),
		)
	}

	await Promise.all(promises)
}
