package com.example.objectionapp

import androidx.compose.ui.graphics.Color
import kotlinx.serialization.SerialName
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

@Serializable
data class Theme(
    @SerialName("tab_bar") val tabBar: TabBar?,
    @SerialName("corner_rounding") val cornerRounding: CornerRounding,
    @SerialName("light_surfaces") val lightSurfaces: HashMap<String, SurfaceTheme>,
    @SerialName("dark_surfaces") val darkSurfaces: HashMap<String, SurfaceTheme>,
    @SerialName("default_light_surface") val defaultLightSurface: SurfaceTheme,
    @SerialName("default_dark_surface") val defaultDarkSurface: SurfaceTheme,
    @SerialName("navigation_surface") val navigationSurface: String?
) {
    fun getInitialObjectId(): String? {
        return getRoots().firstOrNull()
    }

    fun getRoots(): List<String> {
        return tabBar?.objects ?: listOf()
    }

    companion object {
        fun testDefault(): Theme {
            return Theme(
                tabBar = null,
                cornerRounding = CornerRounding.ROUND,
                lightSurfaces = hashMapOf(),
                darkSurfaces = hashMapOf(),
                navigationSurface = null,
                defaultDarkSurface = SurfaceTheme(
                    backgroundColor1 = ColorData(0, 0, 0, 255),
                    backgroundColor2 = ColorData(20, 20, 20, 255),
                    backgroundColor3 = ColorData(30, 30, 30, 255),
                    backgroundColor4 = ColorData(40, 40, 40, 255),

                    foregroundColor1 = ColorData(255, 255, 255, 255),
                    foregroundColor2 = ColorData(255, 255, 255, 255),
                    foregroundColor3 = ColorData(255, 255, 255, 255),
                    foregroundColor4 = ColorData(255, 255, 255, 255),

                    primaryColor1 = ColorData(63, 136, 226, 210),
                    primaryColor2 = ColorData(63, 136, 226, 170),
                    primaryColor3 = ColorData(63, 136, 226, 140),
                    primaryColor4 = ColorData(63, 136, 226, 80),

                    glowColor = null
                ),
                defaultLightSurface = SurfaceTheme(
                    backgroundColor1 = ColorData(0, 0, 0, 255),
                    backgroundColor2 = ColorData(20, 20, 20, 255),
                    backgroundColor3 = ColorData(30, 30, 30, 255),
                    backgroundColor4 = ColorData(40, 40, 40, 255),

                    foregroundColor1 = ColorData(255, 255, 255, 255),
                    foregroundColor2 = ColorData(255, 255, 255, 255),
                    foregroundColor3 = ColorData(255, 255, 255, 255),
                    foregroundColor4 = ColorData(255, 255, 255, 255),

                    primaryColor1 = ColorData(63, 136, 226, 210),
                    primaryColor2 = ColorData(63, 136, 226, 170),
                    primaryColor3 = ColorData(63, 136, 226, 140),
                    primaryColor4 = ColorData(63, 136, 226, 80),

                    glowColor = null
                )
            )
        }
    }
}

@Serializable
data class SurfaceTheme(
    @SerialName("background_color_1") val backgroundColor1: ColorData,
    @SerialName("background_color_2") val backgroundColor2: ColorData,
    @SerialName("background_color_3") val backgroundColor3: ColorData,
    @SerialName("background_color_4") val backgroundColor4: ColorData,

    @SerialName("foreground_color_1") val foregroundColor1: ColorData,
    @SerialName("foreground_color_2") val foregroundColor2: ColorData,
    @SerialName("foreground_color_3") val foregroundColor3: ColorData,
    @SerialName("foreground_color_4") val foregroundColor4: ColorData,

    @SerialName("primary_color_1") val primaryColor1: ColorData,
    @SerialName("primary_color_2") val primaryColor2: ColorData,
    @SerialName("primary_color_3") val primaryColor3: ColorData,
    @SerialName("primary_color_4") val primaryColor4: ColorData,

    @SerialName("glow_color") val glowColor: ColorData?
)

@Serializable
enum class CornerRounding {
    @SerialName("sharp")
    SHARP,

    @SerialName("round")
    ROUND,

    @SerialName("extra_round")
    EXTRA_ROUND,
}

@Serializable
data class TabBar(
    @SerialName("objects") val objects: List<String>
)
