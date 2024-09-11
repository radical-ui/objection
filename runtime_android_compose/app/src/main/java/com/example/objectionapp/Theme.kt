package com.example.objectionapp

import android.graphics.BlurMaskFilter
import androidx.compose.foundation.background
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.graphics.drawOutline
import androidx.compose.ui.graphics.drawscope.drawIntoCanvas
import androidx.compose.ui.unit.dp
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator

@Serializable
data class ColorData(
    val red: Int, val green: Int, val blue: Int, val alpha: Int
) {
    fun intoColor(): Color {
        return Color(red = red, green = green, blue = blue, alpha = alpha)
    }

    fun adjustBrightness(ratio: Double): ColorData {
        val amount = (ratio * 255).toInt()

        return ColorData(this.red - amount, this.green - amount, this.blue - amount, this.alpha)
    }

    fun setAlphaRatio(ratio: Double): ColorData {
        return ColorData(this.red, this.green, this.blue, (alpha * 255).toInt())
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
}

@Serializable
data class SurfaceTheme(
    @SerialName("background") val background: BackgroundData,
    @SerialName("minimal_background") val minimalBackground: BackgroundData,

    @SerialName("selection_color") val selectionColor: ColorData,
    @SerialName("minimal_selection_color") val minimalSelectionColor: ColorData,

    @SerialName("foreground_color") val foreground: ColorData,
    @SerialName("minimal_foreground_color") val minimalForegroundColor: ColorData,

    @SerialName("shadow") val shadow: SurfaceShadow?,
    @SerialName("border") val border: SurfaceBorder?,
)


@Serializable
data class SurfaceShadow(
    @SerialName("color") val color: ColorData,
    @SerialName("blur") val blur: Float,
    @SerialName("spread") val spread: Float,
    @SerialName("offset_x") val offsetX: Float,
    @SerialName("offset_y") val offsetY: Float,
)

fun Modifier.dropShadow(shape: Shape, surfaceShadow: SurfaceShadow) = this.drawBehind {
    val spread = surfaceShadow.spread.dp
    val blur = surfaceShadow.blur.dp
    val color = surfaceShadow.color.intoColor()
    val offsetX = surfaceShadow.offsetX.dp
    val offsetY = surfaceShadow.offsetY.dp

    val shadowSize = Size(size.width + spread.toPx(), size.height + spread.toPx())
    val shadowOutline = shape.createOutline(shadowSize, layoutDirection, this)

    val paint = Paint()
    paint.color = color

    if (blur.toPx() > 0) {
        paint.asFrameworkPaint().apply {
            maskFilter = BlurMaskFilter(blur.toPx(), BlurMaskFilter.Blur.NORMAL)
        }
    }

    drawIntoCanvas { canvas ->
        canvas.save()
        canvas.translate(offsetX.toPx(), offsetY.toPx())
        canvas.drawOutline(shadowOutline, paint)
        canvas.restore()
    }
}

@Serializable
data class SurfaceBorder(
    @SerialName("color") val color: ColorData,
    @SerialName("width") val width: Float,
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

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class BackgroundData {
    @Serializable
    data class ColorDef(val color: ColorData)

    @Serializable
    data class GradientDef(val from: ColorData, val to: ColorData, val angle: Float)

    @Serializable
    @SerialName("color")
    data class Color(val def: ColorDef) : BackgroundData()

    @Serializable
    @SerialName("gradient")
    data class Gradient(val def: GradientDef) : BackgroundData()

    fun applyToModifier(modifier: Modifier): Modifier {
        return when (this) {
            is BackgroundData.Color -> {
                modifier.background(this.def.color.intoColor())
            }

            is BackgroundData.Gradient -> {
                modifier.background(
                    LinearGradient(
                        colors = listOf(this.def.from.intoColor(), this.def.to.intoColor()),
                        stops = listOf(0f, 1f),
                        angleInDegrees = this.def.angle
                    )
                )
            }
        }
    }
}

@Serializable
data class TabBar(
    @SerialName("objects") val objects: List<String>
)
