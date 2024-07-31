import { ComponentRender } from './component.tsx'
import { Component, frontier, React } from './deps.ts'

export interface Color {
	opacity: number
	kind: ColorType
}

export type ColorType = 'Primary' | 'Fore' | 'DecorationFore' | 'Base' | 'Danger' | 'Warn' | 'Success'
export type SelectionMode = 'OptIn' | 'OptOut'

export interface Theme {
	darkPalette: ColorPalette
	defaultFont?: string
	fancyFont?: string
	lightPalette: ColorPalette
	roundBase: boolean
	selectionMode: SelectionMode
	windowScrolling: boolean
}

export interface ColorPalette {
	base: ColorDefinition
	danger: ColorDefinition
	decorationFore: ColorDefinition
	fore: ColorDefinition
	notice: ColorDefinition
	primary: ColorDefinition
	secondary: ColorDefinition
	success: ColorDefinition
	warn: ColorDefinition
}

export interface ColorDefinition {
	blue: number
	green: number
	red: number
}

/**
 * A theme manager for all components. Currently the theme of all components, regardless of location are affected, but this is expected to change
 * to where only child components are affected.
 *
 * @component
 */
export interface ThemeManager {
	theme: Theme
	body: Component
}

export function ThemeManagerRender(props: ThemeManager) {
	frontier.setupDynamicTheme({
		lightPalette: convertPalette(props.theme.lightPalette),
		darkPalette: convertPalette(props.theme.darkPalette),
		options: {
			roundBase: props.theme.roundBase,
			selectionMode: props.theme.selectionMode === 'OptIn' ? 'opt-in' : 'opt-out',
			windowScrolling: props.theme.windowScrolling,
			defaultFont: props.theme.defaultFont ?? undefined,
			fancyFont: props.theme.fancyFont ?? undefined,
		},
	})

	return <ComponentRender {...props.body} />
}

function convertPalette(palette: ColorPalette): frontier.Palette {
	return {
		primary: toColor(palette.primary),
		secondary: toColor(palette.secondary),
		base: toColor(palette.base),
		fore: toColor(palette.fore),
		decorationFore: toColor(palette.decorationFore),
		danger: toColor(palette.danger),
		warn: toColor(palette.warn),
		success: toColor(palette.success),
		notice: toColor(palette.notice),
	}
}

function toColor(def: ColorDefinition) {
	return [def.red, def.green, def.blue] as frontier.Color
}
