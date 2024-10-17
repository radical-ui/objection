import { ImageKind } from './build_config.ts'
import { dtils } from './deps.ts'

export class Image {
	path: string
	kind: ImageKind

	constructor(path: string, kind: ImageKind) {
		this.path = path
		this.kind = kind
	}

	async resize(width: number | null, height: number | null) {
		if (width === null && height === null) throw new Error('Both the width and height cannot be null')

		const outPath = await Deno.makeTempFile()
		const w = width ?? -1
		const h = height ?? -1

		await dtils.execCapture(['ffmpeg', '-i', this.path, '-vf', `scale=${w}:${h}`, outPath])

		return new Image(outPath)
	}

	async copyTo(path: string) {
		try {
			await Deno.remove(path)
		} catch (_) {
			// it's ok if this fails
		}

		await Deno.copyFile(this.path, path)
	}
}
