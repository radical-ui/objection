import { React } from './runtime/deps.ts'
import { Component } from './runtime/types.ts'

export * from './components/mod.tsx'
export * from './runtime/types.ts'

export function start(syncUrl: string, initialComponent: Component, ComponentRenderer: React.FC<Component>) {
	React.render(React.createElement(ComponentRenderer, initialComponent), document.getElementById('root')!)
}
