import { getContext, setContext } from 'svelte'
import type { Size, Color } from './element_type'

export type ThemeDataSizing = {
	xs: '0.25rem' // 4px
	sm: '0.5rem' // 8px
	md: '1rem' // 16px
	lg: '1.5rem' // 24px
	xl: '2rem' // 32px
	'2xl': '3rem' // 48px
	'3xl': '4rem' // 64px
	'4xl': '6rem' // 96px
	full: '100%'
}

export type ThemeContext = {
	sizeToCSS: (value: Size | undefined) => string | undefined
	colorToCSS: (value: Color | undefined) => string | undefined
}

const CSS_CONTEXT_KEY = Symbol('css-context')

// Default size mappings
const defaultSizeMap: Record<string, string> = {
	xs: '0.25rem', // 4px
	sm: '0.5rem', // 8px
	md: '1rem', // 16px
	lg: '1.5rem', // 24px
	xl: '2rem', // 32px
	'2xl': '3rem', // 48px
	'3xl': '4rem', // 64px
	'4xl': '6rem', // 96px
	full: '100%',
}

// Default color mappings
const defaultColorMap: Record<string, string> = {
	primary: '#3b82f6', // blue-500
	success: '#10b981', // emerald-500
	danger: '#ef4444', // red-500
	warning: '#f59e0b', // amber-500
	info: '#06b6d4', // cyan-500
	foreground: '#0f172a', // slate-900
	background: '#ffffff', // white
	border: '#e2e8f0', // slate-200
	focus: '#3b82f6', // blue-500
	muted: '#f8fafc', // slate-50
	muted_foreground: '#64748b', // slate-500
}

export function createCSSContext(
	sizeMap: Record<string, string> = defaultSizeMap,
	colorMap: Record<string, string> = defaultColorMap
): ThemeContext {
	const sizeToCSS = (value: Size | undefined): string | undefined => {
		if (value === undefined) return undefined
		if (typeof value === 'number') return `${value}px`
		if (typeof value === 'string' && sizeMap[value]) return sizeMap[value]
		return undefined
	}

	const colorToCSS = (value: Color | undefined): string | undefined => {
		if (value === undefined) return undefined
		if (Array.isArray(value)) {
			const [r, g, b, a] = value
			return `rgba(${r}, ${g}, ${b}, ${a})`
		}
		if (typeof value === 'string' && colorMap[value]) return colorMap[value]
		return undefined
	}

	return { sizeToCSS, colorToCSS }
}

export function setCSSContext(context: ThemeContext): void {
	setContext(CSS_CONTEXT_KEY, context)
}

export function getCSSContext(): ThemeContext {
	const context = getContext<ThemeContext>(CSS_CONTEXT_KEY)
	if (!context) {
		throw new Error('CSS context not found. Make sure to call setCSSContext in a parent component.')
	}
	return context
}
