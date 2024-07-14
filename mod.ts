import { React } from './runtime/deps.ts'
import { Component } from './runtime/types.ts'

export * from './components/mod.tsx'
export * from './runtime/types.ts'

export function start(syncUrl: string, initialComponent: Component, componentRenderer: (component: Component) => React.ReactNode) {
	// TODO
}
