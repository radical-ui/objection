package com.example.objectionapp

import androidx.compose.material.icons.Icons
import androidx.compose.material3.Icon
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.graphics.vector.ImageVector

@Composable
fun StandardIcon(name: String, outline: Boolean = false) {
    val theme = useTheme()

    val tone = getTone(outline, theme.value.cornerRounding)
    val icon: ImageVector? = remember(name) {
        try {
            val cl = Class.forName("androidx.compose.material.icons.$tone.${name}Kt")
            val method = cl.declaredMethods.first()
            method.invoke(null, Icons.Filled) as ImageVector
        } catch (_: Throwable) {
            null
        }
    }

    if (icon != null) {
        Icon(icon, "$name icon, $tone")
    }
}

private fun getTone(outline: Boolean, rounding: CornerRounding): String {
    if (outline) return "outlined"

    if (rounding == CornerRounding.ROUND) return "filled"
    if (rounding == CornerRounding.EXTRA_ROUND) return "rounded"
    if (rounding == CornerRounding.SHARP) return "sharp"

    println("Unreachable")
    return "filled"
}
