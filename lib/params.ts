import { BuildConfig } from './build_config.ts'

export class BuildParams {
	config: BuildConfig
	items: Record<string, unknown> = {}

	constructor(config: BuildConfig) {
		this.config = config
	}

	setAndParseValue(id: string, value: string) {
		const type = this.config.params[id]?.type
		if (!type) throw new Error(`Param "${id}" does not exist`)

		this.items[id] = value
	}

	getColor(id: string) {
	}
}
