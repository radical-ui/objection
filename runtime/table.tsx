import { Component, ComponentRender } from './component.tsx'
import { React } from './deps.ts'

/**
 * TODO
 *
 * **Example**
 *
 * ```rust Table::new() .column("Id") .expanded_column("Name") .column("") .rows(Vec::from([ Vec::<Component>::from([ Label::new("82").into(), Label::new("Jason").into(), Button::new("View").size(ButtonSize::Small).into() ]), Vec::<Component>::from([ Label::new("84").into(), Label::new("James").into(), Button::new("View").size(ButtonSize::Small).into() ]), Vec::<Component>::from([ Label::new("103").into(), Label::new("Jeehoshofat Bartholemew, Duke of Northumberland, King of \"The Rose Garden\", the sixteenth").into(), Button::new("View").size(ButtonSize::Small).into() ]), ])) ```
 *
 * @component
 */
export interface Table {
	columns: TableColumn[]
	rows: Component[][]
}
export interface TableColumn {
	expand: boolean
	name: string
}

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
								<div
									class={`bg-fore-5 h-full w-full py-8 px-14 text-fore-50 font-semibold ${
										!col.expand ? 'text-center' : ''
									}`}
								>
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
