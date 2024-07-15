import { colors } from './deps.ts'

let lastPrintLength = 0

const encoder = new TextEncoder()

export function print(text: string) {
	const thisPrintLength = colors.stripAnsiCode(text).length
	const buffer = lastPrintLength > thisPrintLength ? lastPrintLength - thisPrintLength : 0
	const spacing = ' '.repeat(buffer)

	Deno.stderr.writeSync(encoder.encode(`${text}${spacing}\r`))

	lastPrintLength = thisPrintLength
}

export function flush() {
	print('')
}
