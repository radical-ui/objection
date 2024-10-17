import { streams } from './deps.ts'

const config = await streams.toJson(Deno.stdin.readable)

export function getConfigValue(id: string): unknown {
}
