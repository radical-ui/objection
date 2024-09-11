package com.example.objectionapp

fun makeTestTheme(tabBarObjects: List<String>? = null): Theme {
    val primarySurface = Pair("primary", SurfaceTheme(
        background = BackgroundData.Color(BackgroundData.ColorDef(blueColorData())),
        minimalBackground = BackgroundData.Color(BackgroundData.ColorDef(blueColorData().setAlphaRatio(0.1))),
        selectionColor = blueColorData(),
        minimalSelectionColor = blueColorData().setAlphaRatio(0.1),
        foreground = grayColorData(1.0),
        minimalForegroundColor = grayColorData(1.0).setAlphaRatio(0.3),
        border = null,
        shadow = null,
    ));

    return Theme(
        tabBar = tabBarObjects?.let { TabBar(it) },
        cornerRounding = CornerRounding.ROUND,
        lightSurfaces = hashMapOf(
            Pair("contrast_1", lightSurface(blueColorData(), 0.9)),
            Pair("contrast_2", lightSurface(blueColorData(), 0.8)),
            Pair("contrast_3", lightSurface(blueColorData(), 0.7)),
            Pair("contrast_4", lightSurface(blueColorData(), 0.6)),
            primarySurface
        ),
        darkSurfaces = hashMapOf(
            Pair("contrast_1", darkSurface(blueColorData(), 0.1)),
            Pair("contrast_2", darkSurface(blueColorData(), 0.2)),
            Pair("contrast_3", darkSurface(blueColorData(), 0.3)),
            Pair("contrast_4", darkSurface(blueColorData(), 0.4)),
            primarySurface
        ),
        navigationSurface = null, // use the default
        defaultDarkSurface = lightSurface(blueColorData(), 0.0),
        defaultLightSurface = darkSurface(blueColorData(), 1.0)
    )
}

fun darkSurface(selectionColor: ColorData, baseBrightness: Double): SurfaceTheme {
    return SurfaceTheme(
        background = BackgroundData.Color(
            def = BackgroundData.ColorDef(grayColorData(baseBrightness))
        ),
        minimalBackground = BackgroundData.Color(
            def = BackgroundData.ColorDef(
                grayColorData(baseBrightness + 0.1)
            )
        ),
        selectionColor = selectionColor,
        minimalSelectionColor = selectionColor,
        foreground = grayColorData(1.0),
        minimalForegroundColor = grayColorData(baseBrightness + 0.4),
        border = null,
        shadow = null,
    )
}

fun lightSurface(selectionColor: ColorData, baseBrightness: Double): SurfaceTheme {
    return SurfaceTheme(
        background = BackgroundData.Color(
            def = BackgroundData.ColorDef(grayColorData(baseBrightness))
        ),
        minimalBackground = BackgroundData.Color(
            def = BackgroundData.ColorDef(
                grayColorData(baseBrightness - 0.1)
            )
        ),
        selectionColor = selectionColor,
        minimalSelectionColor = selectionColor,
        foreground = grayColorData(0.0),
        minimalForegroundColor = grayColorData(baseBrightness - 0.4),
        border = null,
        shadow = null,
    )
}

fun grayColorData(brightness: Double): ColorData {
    val amount = (brightness * 255).toInt()

    return ColorData(amount, amount, amount, 255)
}

fun blueColorData(): ColorData {
    return ColorData(27, 167, 240, 255)
}