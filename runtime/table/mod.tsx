import { React, Table } from 'runtime'
import { ComponentRender } from '../mod.tsx'

export function TableRender(props: Table) {
	const columnStyles = props.columns.map((col) => {
		return col.expand ? '1fr' : 'auto'
	})

	return (
		<div class='rounded border border-fore-10'>
			<div class='bg-fore-10 rounded overflow-hidden'>
				<div class='grid gap-2' style={{ gridTemplateColumns: columnStyles.join(' ') }}>
					{props.columns.map((col) => {
						return (
							<div class='bg-base'>
								<div class={`bg-fore-5 h-full w-full py-8 px-14 text-fore-50 font-semibold ${!col.expand ? 'text-center' : ''}`}>
									{col.name}
								</div>
							</div>
						)
					})}
					{props.rows.map((row) =>
						row.map((cell, index) => (
							<div class={`bg-base py-8 px-14 flex items-center ${!props.columns[index]?.expand ? 'justify-center' : ''}`}>
								<ComponentRender {...cell} />
							</div>
						))
					)}
				</div>
			</div>
		</div>
	)
}
