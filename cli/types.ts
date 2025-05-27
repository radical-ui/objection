export type BuildMode = 'client' | 'server'

export function isBuildMode(input: string): input is BuildMode {
	return input === 'client' || input === 'server'
}
