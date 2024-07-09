import { Md5 } from 'https://deno.land/x/dtils@2.6.1/mod.ts'
import { colors, dtils, streamUtils } from './deps.ts'

export function successLog(verb: string, noun: string) {
	console.log(colors.green(colors.bold(verb)), colors.gray(noun))
}

export function errorLog(message: string) {
	console.log(colors.red(colors.bold('Error')), colors.gray(message))
}

export function warnLog(message: string) {
	console.log(colors.yellow(colors.bold('Warn')), colors.gray(message))
}

export async function pipeCommand(command: string, input: string): Promise<string> {
	const shell = Deno.env.get('SHELL')
	if (!shell) throw new Error('Expected to find a SHELL env var')

	const proc = new Deno.Command(shell, { args: ['-c', command], stdin: 'piped', stdout: 'piped', stderr: 'inherit' }).spawn()
	const writer = proc.stdin.getWriter()
	await writer.write(new TextEncoder().encode(input))
	writer.close()

	const [output, status] = await Promise.all([
		streamUtils.toText(proc.stdout),
		proc.status,
	])

	if (!status.success) throw new Error('Command failed')

	return output
}

export async function formatRs(code: string) {
	return await pipeCommand('rustfmt', code)
}

export async function formatTs(code: string) {
	return await pipeCommand('deno fmt -', code)
}

export async function perhapsWrite(file: string, text: string) {
	const existingText = await dtils.readText(file) ?? ''
	if (existingText.trim().length && !confirm(`Content at ${file} already exists. Overwrite?`)) return warnLog(`skipped writing to ${file}`)

	await dtils.writeText(file, text)
}

export async function getWithCache(url: string) {
	const key = Md5.hash(url)
	const path = `target/${key}.json`

	if (await dtils.exists(path)) return await dtils.readJson(path)

	successLog('Download', url)
	const data = await fetch(url).then((res) => res.json())
	await dtils.writeJson(path, data)

	return data
}
