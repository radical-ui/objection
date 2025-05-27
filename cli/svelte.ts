import { file, type BunPlugin } from 'bun'
import { compile } from 'svelte/compiler'

export type SveltePluginOptions = {
	isServer?: boolean
	onCss?(css: string): void
	isDev?: boolean
}

export function sveltePlugin(params: SveltePluginOptions): BunPlugin {
	return {
		name: 'svelte loader',
		setup(builder) {
			builder.onLoad({ filter: /\.svelte$/ }, async function ({ path }) {
				const source = await file(path).text()

				const result = compile(source, {
					filename: path,
					generate: params.isServer ? 'server' : 'client',
					dev: params.isDev,
				})

				if (result.css && params.onCss) params.onCss(result.css.code)

				return {
					contents: result.js.code,
					loader: 'ts',
				}
			})
		},
	}
}
