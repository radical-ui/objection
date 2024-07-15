import { Component, ComponentRender } from './component.tsx'
import { React } from './deps.ts'

const svg =
	"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32' width='32' height='32' fill='none' stroke='rgb(100, 100, 100, 0.1)'><path d='M0 .5H31.5V32'/></svg>"

/**
 * A ui-decorated box for displaying content
 *
 * @component
 */
export interface PreviewBox {
	child: Component
	title: string
}

export function PreviewBoxRender(params: PreviewBox) {
	return (
		<div class='border border-fore-10 rounded-md flex flex-col h-400'>
			<div class='px-15 py-5 bg-fore-5 font-semibold text-fore-40'>{params.title}</div>
			<div class='h-2 bg-fore-10' />
			<div class='flex-1 bg-base min-h-0' style={`background-image: url("data:image/svg+xml,${encodeURIComponent(svg)}")`}>
				{<ComponentRender {...params.child} />}
			</div>
		</div>
	)
}
