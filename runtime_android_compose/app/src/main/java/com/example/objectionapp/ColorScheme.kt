package com.example.objectionapp

import androidx.compose.ui.graphics.Color
import kotlinx.serialization.Serializable

@Serializable
data class ColorScheme(
	val primary: ColorData? = null,
	val onPrimary: ColorData? = null,
	val primaryContainer: ColorData? = null,
	val onPrimaryContainer: ColorData? = null,
	val inversePrimary: ColorData? = null,
	val secondary: ColorData? = null,
	val onSecondary: ColorData? = null,
	val secondaryContainer: ColorData? = null,
	val onSecondaryContainer: ColorData? = null,
	val tertiary: ColorData? = null,
	val onTertiary: ColorData? = null,
	val tertiaryContainer: ColorData? = null,
	val onTertiaryContainer: ColorData? = null,
	val background: ColorData? = null,
	val onBackground: ColorData? = null,
	val surface: ColorData? = null,
	val onSurface: ColorData? = null,
	val surfaceVariant: ColorData? = null,
	val onSurfaceVariant: ColorData? = null,
	val surfaceTint: ColorData? = null,
	val inverseSurface: ColorData? = null,
	val inverseOnSurface: ColorData? = null,
	val error: ColorData? = null,
	val onError: ColorData? = null,
	val errorContainer: ColorData? = null,
	val onErrorContainer: ColorData? = null,
	val outline: ColorData? = null,
	val outlineVariant: ColorData? = null,
	val scrim: ColorData? = null,
	val surfaceBright: ColorData? = null,
	val surfaceContainer: ColorData? = null,
	val surfaceContainerHigh: ColorData? = null,
	val surfaceContainerHighest: ColorData? = null,
	val surfaceContainerLow: ColorData? = null,
	val surfaceContainerLowest: ColorData? = null,
	val surfaceDim: ColorData? = null,
)

fun fillLightDefaults(scheme: ColorScheme): androidx.compose.material3.ColorScheme {
	return androidx.compose.material3.ColorScheme(
		primary = scheme.primary?.intoColor() ?: Color(red = 103, green = 80, blue = 164, alpha = 255),
		onPrimary = scheme.onPrimary?.intoColor() ?: Color(red = 255, green = 255, blue = 255, alpha = 255),
		primaryContainer = scheme.primaryContainer?.intoColor() ?: Color(
			red = 234,
			green = 221,
			blue = 255,
			alpha = 255
		),
		onPrimaryContainer = scheme.onPrimaryContainer?.intoColor() ?: Color(
			red = 33,
			green = 0,
			blue = 93,
			alpha = 255
		),
		inversePrimary = scheme.inversePrimary?.intoColor() ?: Color(
			red = 208,
			green = 188,
			blue = 255,
			alpha = 255
		),
		secondary = scheme.secondary?.intoColor() ?: Color(red = 98, green = 91, blue = 113, alpha = 255),
		onSecondary = scheme.onSecondary?.intoColor() ?: Color(red = 255, green = 255, blue = 255, alpha = 255),
		secondaryContainer = scheme.secondaryContainer?.intoColor() ?: Color(
			red = 232,
			green = 222,
			blue = 248,
			alpha = 255
		),
		onSecondaryContainer = scheme.onSecondaryContainer?.intoColor() ?: Color(
			red = 29,
			green = 25,
			blue = 43,
			alpha = 255
		),
		tertiary = scheme.tertiary?.intoColor() ?: Color(red = 125, green = 82, blue = 96, alpha = 255),
		onTertiary = scheme.onTertiary?.intoColor() ?: Color(red = 255, green = 255, blue = 255, alpha = 255),
		tertiaryContainer = scheme.tertiaryContainer?.intoColor() ?: Color(
			red = 255,
			green = 216,
			blue = 228,
			alpha = 255
		),
		onTertiaryContainer = scheme.onTertiaryContainer?.intoColor() ?: Color(
			red = 49,
			green = 17,
			blue = 29,
			alpha = 255
		),
		background = scheme.background?.intoColor() ?: Color(red = 254, green = 247, blue = 255, alpha = 255),
		onBackground = scheme.onBackground?.intoColor() ?: Color(red = 29, green = 27, blue = 32, alpha = 255),
		surface = scheme.surface?.intoColor() ?: Color(red = 254, green = 247, blue = 255, alpha = 255),
		onSurface = scheme.onSurface?.intoColor() ?: Color(red = 29, green = 27, blue = 32, alpha = 255),
		surfaceVariant = scheme.surfaceVariant?.intoColor() ?: Color(
			red = 231,
			green = 224,
			blue = 236,
			alpha = 255
		),
		onSurfaceVariant = scheme.onSurfaceVariant?.intoColor() ?: Color(
			red = 73,
			green = 69,
			blue = 79,
			alpha = 255
		),
		surfaceTint = scheme.surfaceTint?.intoColor() ?: Color(red = 103, green = 80, blue = 164, alpha = 255),
		inverseSurface = scheme.inverseSurface?.intoColor() ?: Color(
			red = 50,
			green = 47,
			blue = 53,
			alpha = 255
		),
		inverseOnSurface = scheme.inverseOnSurface?.intoColor() ?: Color(
			red = 245,
			green = 239,
			blue = 247,
			alpha = 255
		),
		error = scheme.error?.intoColor() ?: Color(red = 179, green = 38, blue = 30, alpha = 255),
		onError = scheme.onError?.intoColor() ?: Color(red = 255, green = 255, blue = 255, alpha = 255),
		errorContainer = scheme.errorContainer?.intoColor() ?: Color(
			red = 249,
			green = 222,
			blue = 220,
			alpha = 255
		),
		onErrorContainer = scheme.onErrorContainer?.intoColor() ?: Color(
			red = 65,
			green = 14,
			blue = 11,
			alpha = 255
		),
		outline = scheme.outline?.intoColor() ?: Color(red = 121, green = 116, blue = 126, alpha = 255),
		outlineVariant = scheme.outlineVariant?.intoColor() ?: Color(
			red = 202,
			green = 196,
			blue = 208,
			alpha = 255
		),
		scrim = scheme.scrim?.intoColor() ?: Color(red = 0, green = 0, blue = 0, alpha = 255),
		surfaceBright = scheme.surfaceBright?.intoColor() ?: Color(
			red = 254,
			green = 247,
			blue = 255,
			alpha = 255
		),
		surfaceContainer = scheme.surfaceContainer?.intoColor() ?: Color(
			red = 243,
			green = 237,
			blue = 247,
			alpha = 255
		),
		surfaceContainerHigh = scheme.surfaceContainerHigh?.intoColor() ?: Color(
			red = 236,
			green = 230,
			blue = 240,
			alpha = 255
		),
		surfaceContainerHighest = scheme.surfaceContainerHighest?.intoColor() ?: Color(
			red = 230,
			green = 224,
			blue = 233,
			alpha = 255
		),
		surfaceContainerLow = scheme.surfaceContainerLow?.intoColor() ?: Color(
			red = 247,
			green = 242,
			blue = 250,
			alpha = 255
		),
		surfaceContainerLowest = scheme.surfaceContainerLowest?.intoColor() ?: Color(
			red = 255,
			green = 255,
			blue = 255,
			alpha = 255
		),
		surfaceDim = scheme.surfaceDim?.intoColor() ?: Color(red = 222, green = 216, blue = 225, alpha = 255),
	)
}

fun fillDarkDefaults(scheme: ColorScheme): androidx.compose.material3.ColorScheme {
	return androidx.compose.material3.ColorScheme(
		primary = scheme.primary?.intoColor() ?: Color(red = 208, green = 188, blue = 255, alpha = 255),
		onPrimary = scheme.onPrimary?.intoColor() ?: Color(red = 56, green = 30, blue = 114, alpha = 255),
		primaryContainer = scheme.primaryContainer?.intoColor() ?: Color(
			red = 79,
			green = 55,
			blue = 139,
			alpha = 255
		),
		onPrimaryContainer = scheme.onPrimaryContainer?.intoColor() ?: Color(
			red = 234,
			green = 221,
			blue = 255,
			alpha = 255
		),
		inversePrimary = scheme.inversePrimary?.intoColor() ?: Color(
			red = 103,
			green = 80,
			blue = 164,
			alpha = 255
		),
		secondary = scheme.secondary?.intoColor() ?: Color(red = 204, green = 194, blue = 220, alpha = 255),
		onSecondary = scheme.onSecondary?.intoColor() ?: Color(red = 51, green = 45, blue = 65, alpha = 255),
		secondaryContainer = scheme.secondaryContainer?.intoColor() ?: Color(
			red = 74,
			green = 68,
			blue = 88,
			alpha = 255
		),
		onSecondaryContainer = scheme.onSecondaryContainer?.intoColor() ?: Color(
			red = 232,
			green = 222,
			blue = 248,
			alpha = 255
		),
		tertiary = scheme.tertiary?.intoColor() ?: Color(red = 239, green = 184, blue = 200, alpha = 255),
		onTertiary = scheme.onTertiary?.intoColor() ?: Color(red = 73, green = 37, blue = 50, alpha = 255),
		tertiaryContainer = scheme.tertiaryContainer?.intoColor() ?: Color(
			red = 99,
			green = 59,
			blue = 72,
			alpha = 255
		),
		onTertiaryContainer = scheme.onTertiaryContainer?.intoColor() ?: Color(
			red = 255,
			green = 216,
			blue = 228,
			alpha = 255
		),
		background = scheme.background?.intoColor() ?: Color(red = 20, green = 18, blue = 24, alpha = 255),
		onBackground = scheme.onBackground?.intoColor() ?: Color(
			red = 230,
			green = 224,
			blue = 233,
			alpha = 255
		),
		surface = scheme.surface?.intoColor() ?: Color(red = 20, green = 18, blue = 24, alpha = 255),
		onSurface = scheme.onSurface?.intoColor() ?: Color(red = 230, green = 224, blue = 233, alpha = 255),
		surfaceVariant = scheme.surfaceVariant?.intoColor() ?: Color(
			red = 73,
			green = 69,
			blue = 79,
			alpha = 255
		),
		onSurfaceVariant = scheme.onSurfaceVariant?.intoColor() ?: Color(
			red = 202,
			green = 196,
			blue = 208,
			alpha = 255
		),
		surfaceTint = scheme.surfaceTint?.intoColor() ?: Color(red = 208, green = 188, blue = 255, alpha = 255),
		inverseSurface = scheme.inverseSurface?.intoColor() ?: Color(
			red = 230,
			green = 224,
			blue = 233,
			alpha = 255
		),
		inverseOnSurface = scheme.inverseOnSurface?.intoColor() ?: Color(
			red = 50,
			green = 47,
			blue = 53,
			alpha = 255
		),
		error = scheme.error?.intoColor() ?: Color(red = 242, green = 184, blue = 181, alpha = 255),
		onError = scheme.onError?.intoColor() ?: Color(red = 96, green = 20, blue = 16, alpha = 255),
		errorContainer = scheme.errorContainer?.intoColor() ?: Color(
			red = 140,
			green = 29,
			blue = 24,
			alpha = 255
		),
		onErrorContainer = scheme.onErrorContainer?.intoColor() ?: Color(
			red = 249,
			green = 222,
			blue = 220,
			alpha = 255
		),
		outline = scheme.outline?.intoColor() ?: Color(red = 147, green = 143, blue = 153, alpha = 255),
		outlineVariant = scheme.outlineVariant?.intoColor() ?: Color(
			red = 73,
			green = 69,
			blue = 79,
			alpha = 255
		),
		scrim = scheme.scrim?.intoColor() ?: Color(red = 0, green = 0, blue = 0, alpha = 255),
		surfaceBright = scheme.surfaceBright?.intoColor() ?: Color(red = 59, green = 56, blue = 62, alpha = 255),
		surfaceContainer = scheme.surfaceContainer?.intoColor() ?: Color(
			red = 33,
			green = 31,
			blue = 38,
			alpha = 255
		),
		surfaceContainerHigh = scheme.surfaceContainerHigh?.intoColor() ?: Color(
			red = 43,
			green = 41,
			blue = 48,
			alpha = 255
		),
		surfaceContainerHighest = scheme.surfaceContainerHighest?.intoColor() ?: Color(
			red = 54,
			green = 52,
			blue = 59,
			alpha = 255
		),
		surfaceContainerLow = scheme.surfaceContainerLow?.intoColor() ?: Color(
			red = 29,
			green = 27,
			blue = 32,
			alpha = 255
		),
		surfaceContainerLowest = scheme.surfaceContainerLowest?.intoColor() ?: Color(
			red = 15,
			green = 13,
			blue = 19,
			alpha = 255
		),
		surfaceDim = scheme.surfaceDim?.intoColor() ?: Color(red = 20, green = 18, blue = 24, alpha = 255),
	)
}
