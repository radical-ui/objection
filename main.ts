const input = getCliInput()
const version = '0.8.0'
const help = `\
Objection CLI

Usage:
  objection [...global options] <command> <runtime> [...configurations]

Global Options:
  --help, -h    Show this help and exit
  --version, -v Print the current version (${version}) and exit

Commands:
  gen           TODO
  preview       TODO
  deploy        TODO`

if (hasFlag('h') || hasFlag('help')) {
	console.log(help)
} else if (hasFlag('v') || hasFlag('version')) {
	console.log(version)
} else {
	const command = getArg()
	if (!command) throw new Error('Expected a command')

	const runtimePath = getRuntimePath()

	if (command == 'gen') {
		throw new Error('Not implemented')
	} else if (command == 'preview') {
		throw new Error('Not implemented')
	} else if (command == 'deploy') {
		throw new Error('Not Implemented')
	} else {
		throw new Error(`Invalid command: ${command}`)
	}
}

function getRuntimePath() {
	const runtimeId = getArg()
	if (!runtimeId) throw new Error('Expected a runtime')

	// TODO dowload repo if it is a github id

	return runtimeId
}

function getArg() {
	for (let index = 0; index < Deno.args.length; index++) {
		const arg = input.args.get(index)
		if (arg !== undefined) {
			input.args.delete(index)
			return arg
		}
	}

	return null
}

function hasFlag(name: string) {
	if (!input.options.has(name)) return false

	input.options.delete(name)
	return true
}

function getOption(name: string) {
	const index = input.options.get(name)
	if (index === undefined) return null

	const nextIndex = index + 1
	const value = input.args.get(nextIndex)
	if (value === undefined) {
		return null
	}

	input.options.delete(name)
	input.args.delete(nextIndex)

	return value
}

interface CliInput {
	args: Map<number, string>
	options: Map<string, number>
}

function getCliInput(): CliInput {
	const options = new Map<string, number>()
	const args = new Map<number, string>()

	for (const indexStr in Deno.args) {
		const index = parseInt(indexStr)
		const inputArg = Deno.args[index]

		if (inputArg.startsWith('--')) options.set(inputArg.slice(2), index)
		else if (inputArg.startsWith('-') && inputArg.length == 2) options.set(inputArg.slice(1), index)
		else args.set(index, inputArg)
	}

	return { args, options }
}
