package com.example.objectionapp

import androidx.compose.ui.graphics.Color
import kotlinx.serialization.Serializable

@Serializable
@IsColor
data class ColorData(
	val red: Int,
	val green: Int,
	val blue: Int,
	val alpha: Int
) {
	fun intoColor(): Color {
		return Color(red = red, green = green, blue = blue, alpha = alpha)
	}
}