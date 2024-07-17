import { Component } from './component.tsx'

export type Color =
	| {
		opacity: number
		type: 'Primary'
	}
	| {
		opacity: number
		type: 'Fore'
	}
	| {
		opacity: number
		type: 'DecorationFore'
	}
	| {
		opacity: number
		type: 'Base'
	}
	| {
		opacity: number
		type: 'Danger'
	}
	| {
		opacity: number
		type: 'Warn'
	}
	| {
		opacity: number
		type: 'Success'
	}
export type ColorType = 'Primary' | 'Fore' | 'DecorationFore' | 'Base' | 'Danger' | 'Warn' | 'Success'
export type SelectionMode = 'OptIn' | 'OptOut'
export type UpdateAction =
	| {
		data: Window
		strategy: 'FullUpdate'
	}
	| {
		/**
		 * @minItems 2
		 * @maxItems 2
		 */
		data: [number, unknown]
		strategy: 'ComponentUpdate'
	}
	| {
		data: Notice
		strategy: 'AddNotice'
	}
export type NoticeStyle = 'Error' | 'Success'

export interface Window {
	root_component: Component
	theme?: Theme
	title: string
}
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
export interface Notice {
	message: string
	style: NoticeStyle
}
