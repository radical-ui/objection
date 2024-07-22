import { Component, ComponentRender } from './component.tsx'
import { frontier, React } from './deps.ts'

export interface Color {
	opacity: number
	type: ColorType
}

export type ColorType = 'Primary' | 'Fore' | 'DecorationFore' | 'Base' | 'Danger' | 'Warn' | 'Success'
export type SelectionMode = 'OptIn' | 'OptOut'

export interface Theme {
	dark_palette: ColorPalette
	default_font?: string
	fancy_font?: string
	light_palette: ColorPalette
	round_base: boolean
	selection_mode: SelectionMode
	window_scrolling: boolean
}

export interface ColorPalette {
	base: ColorDefinition
	danger: ColorDefinition
	decoration_fore: ColorDefinition
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
		lightPalette: convertPalette(props.theme.light_palette),
		darkPalette: convertPalette(props.theme.dark_palette),
		options: {
			roundBase: props.theme.round_base,
			selectionMode: props.theme.selection_mode === 'OptIn' ? 'opt-in' : 'opt-out',
			windowScrolling: props.theme.window_scrolling,
			defaultFont: props.theme.default_font ?? undefined,
			fancyFont: props.theme.fancy_font ?? undefined,
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
		decorationFore: toColor(palette.decoration_fore),
		danger: toColor(palette.danger),
		warn: toColor(palette.warn),
		success: toColor(palette.success),
		notice: toColor(palette.notice),
	}
}

function toColor(def: ColorDefinition) {
	return [def.red, def.green, def.blue] as frontier.Color
}
