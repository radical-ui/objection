import { dtils } from './deps.ts'
import { getWithCache } from './utils.ts'

type Icons = Record<string, string>
type Tags = Record<string, string[]>

export async function fetchIcons() {
	try {
		await Deno.remove('www/icons', { recursive: true })
	} catch (_) {
		// ignore
	}

	const MDI_URL = 'https://pictogrammers.com/data/mdi-7.4.47.json'

	const icons: Icons = {}
	const tags: Tags = {}

	await getMdi(MDI_URL, icons, tags)

	for (const icon in icons) {
		const svg = icons[icon]

		await dtils.writeText(`www/icons/${icon}.svg`, svg)
	}

	await dtils.writeJson('www/icons/tags.json', tags, { separator: '\t' })
}

async function getMdi(url: string, icons: Icons, tags: Tags) {
	const json = await getWithCache(url)

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
