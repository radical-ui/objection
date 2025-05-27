import { elementPickerPlugin } from './element_picker'
import pathUtils from 'path'
import { sveltePlugin } from './svelte'

export type BundleParams = {
	sdkPath: string
	specificElements: string[] | null
	outDir: string
}

export async function build(params: BundleParams) {
	let combinedCss = ''

	const [hydrationClientResult, standaloneClientResult, ssrWorkerResult] = await Promise.all([
		Bun.build({
			target: 'browser',
			entrypoints: ['./main/hydration_client.ts'],
			plugins: [
				await elementPickerPlugin(params.sdkPath, params.specificElements),
				sveltePlugin({ onCss: css => (combinedCss += css) }),
			],
		}),
		Bun.build({
			target: 'browser',
			entrypoints: ['./main/standalone_client.ts'],
			plugins: [await elementPickerPlugin(params.sdkPath, params.specificElements), sveltePlugin({})],
		}),
		Bun.build({
			target: 'bun',
			entrypoints: ['./main/ssr_worker.ts'],
			plugins: [await elementPickerPlugin(params.sdkPath, params.specificElements), sveltePlugin({ isServer: true })],
		}),
	])

	if (!hydrationClientResult.success || !standaloneClientResult.success || !ssrWorkerResult.success) throw new Error('Build failed')
	if (!hydrationClientResult.outputs[0] || !standaloneClientResult.outputs[0] || !ssrWorkerResult.outputs[0]) {
		throw new Error('Build does not have expected output')
	}

	await Bun.write(pathUtils.join(params.outDir, 'hydration_client.js'), hydrationClientResult.outputs[0])
	await Bun.write(pathUtils.join(params.outDir, 'standalone_client.js'), standaloneClientResult.outputs[0])
	await Bun.write(pathUtils.join(params.outDir, 'ssr_worker.js'), ssrWorkerResult.outputs[0])
	await Bun.write(pathUtils.join(params.outDir, 'combined.css'), combinedCss)
}
