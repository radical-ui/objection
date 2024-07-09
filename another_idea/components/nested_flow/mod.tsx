import { Component, NestedFlow, NestedFlowItem, React } from 'runtime'
import { ComponentRender } from '../mod.tsx'

interface Block {
	header: string | null
	chunks: Chunk[]
}

interface Chunk {
	header: string
	content: Component
}

const getBlocks = (items: NestedFlowItem[]) => {
	const blocks: Block[] = [{ header: null, chunks: [] }]

	for (const item of items) {
		const lastBlock = blocks[blocks.length - 1]

		if (item.type === 'Header') {
			if (!lastBlock.header) lastBlock.header = item.text
			else blocks.push({ header: item.text, chunks: [] })

			continue
		}

		if (item.type === 'Content') {
			lastBlock.chunks.push({ header: item.header_text, content: item.content })
			continue
		}
	}

	return blocks
}

export function NestedFlowRender(props: NestedFlow) {
	const defaultIndentAmount = props.indent ? 40 : 0
	const blocks = getBlocks(props.items)

	return (
		<div class={`pl-${defaultIndentAmount} flex flex-col gap-40`}>
			{blocks.map((block) => {
				const indentAmount = block.header ? defaultIndentAmount : 0

				return (
					<div class='flex flex-col gap-30'>
						{block.header && <h2 class='text-xl font-semibold text-fore-40'>{block.header}</h2>}
						{block.chunks.map((chunk) => {
							return (
								<div class={`pl-${indentAmount} w-${500 + indentAmount} flex flex-col gap-30`}>
									<div class='flex gap-10 items-center'>
										<h3 class='font-semibold text-primary'>{chunk.header}</h3>
										<div class='h-2 flex-1 bg-primary' />
									</div>
									<div>
										<ComponentRender {...chunk.content} />
									</div>
								</div>
							)
						})}
					</div>
				)
			})}
		</div>
	)
}
