package com.example.objectionapp

import androidx.compose.ui.graphics.Color

class Controller(
    val wsUrl: String,
    val appName: String,

    val noInternetHeader: String,
    val noInternetContent: String,
    val errorHeader: String,

    val lightBackgroundColor: Color,
    val darkBackgroundColor: Color,
    val lightForegroundColor: Color,
    val darkForegroundColor: Color
) {
    val session = Session()
    val logger = Logger(session, scope = appName)
    val bridge = Bridge(logger = logger.scope("Bridge"), session)
    val objectStore = ObjectStore(bridge, logger = logger.scope("ObjectStore"), Object.serializer())

    companion object {
        fun fromConstants(): Controller {
            return Controller(
                wsUrl = wsUrl,
                appName = appName,
                noInternetHeader = noInternetHeader,
                noInternetContent = noInternetContent,
                errorHeader = errorHeader,
                lightBackgroundColor = lightBackgroundColor,
                darkBackgroundColor = darkBackgroundColor,
                lightForegroundColor = lightForegroundColor,
                darkForegroundColor = darkForegroundColor,
            )
        }
    }
}