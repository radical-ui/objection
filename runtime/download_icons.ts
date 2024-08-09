// deno-lint-ignore-file no-external-import

import * as dtils from 'https://deno.land/x/dtils@2.6.1/mod.ts'
import { encodeHex } from 'jsr:@std/encoding@1.0.1/hex'

type Icons = Record<string, string>
type Tags = Record<string, string[]>

interface Asset {
	sha256: string
	webPath: string
	localPath: string
}

try {
	await Deno.remove('runtime/public/icons', { recursive: true })
} catch (_) {
	// ignore
}

const MDI_URL = 'https://pictogrammers.com/data/mdi-7.4.47.json'

const icons: Icons = {}
const tags: Tags = {}
const assets: Asset[] = []

await getMdi(MDI_URL, icons, tags)

for (const icon in icons) {
	const svg = icons[icon]
	const iconPath = `icons/${icon}.svg`

	const hashBuffer = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(svg))
	assets.push({ localPath: iconPath, webPath: iconPath, sha256: encodeHex(hashBuffer) })

	await dtils.writeText(`runtime/${iconPath}`, svg)
}

const tagsJson = dtils.jsonEncode(tags, '\t')
const tagsHashBuffer = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(tagsJson))

assets.push({ localPath: 'icons/tags.json', webPath: 'icons/tags.json', sha256: encodeHex(tagsHashBuffer) })

await dtils.writeJson('runtime/icon_index.json', assets, { separator: '\t' })
await dtils.writeText('runtime/icons/tags.json', tagsJson)

async function getMdi(url: string, icons: Icons, tags: Tags) {
	const json = await fetch(url).then((res) => res.json())

	for (const raw of json.i) {
		const name = `mdi-${raw.n}`
		icons[name] = getSvg(raw.p)

		for (const tag of raw.st) {
			const existingList = tags[tag]

			if (existingList) existingList.push(name)
			else tags[tag] = [name]
		}
	}
}

function getSvg(path: string): string {
	const openingTag = `<svg xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 24 24">`
	const pathHtml = `<path d="${path}" fill="currentColor" />`
	const closingTag = `</svg>`

	return `${openingTag}${pathHtml}${closingTag}`
}
