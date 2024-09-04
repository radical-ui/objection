package com.example.objectionapp

import android.os.Parcel
import android.os.Parcelable
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
    val objectStore = ObjectStore(bridge, logger = logger.scope("ObjectStore"))

    companion object {
        fun default(): Controller {
            return Controller(
                wsUrl = "ws://10.0.2.2:8000/ui.ws",
                appName = "Objection App",
                noInternetHeader = "No Internet",
                noInternetContent = "This app requires an internet connection to function",
                errorHeader = "Uh oh",
                lightBackgroundColor = Color(red = 240, green = 240, blue = 255),
                darkBackgroundColor = Color(red = 12, green = 12, blue = 20),
                lightForegroundColor = Color(red = 12, green = 12, blue = 20),
                darkForegroundColor = Color(red = 240, green = 240, blue = 255),
            )
        }
    }
}