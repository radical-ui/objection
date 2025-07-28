import type { BunPlugin } from 'bun'

export function gatherExamplesPlugin(sdkPath: string): BunPlugin {
	return {
		name: 'gather-examples',
		setup(build) {
			build.onLoad({ filter: /\.example\.ts$/ }, async ({ path }) => {
				const content = await Bun.file(path).text()
				return { contents: content }
			})
		},
	}
}
