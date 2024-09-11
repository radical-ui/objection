package com.example.objectionapp

import androidx.compose.foundation.background
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.State
import androidx.compose.runtime.compositionLocalOf
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.sp
import androidx.compose.ui.zIndex
import androidx.navigation.NavHostController
import androidx.navigation.compose.rememberNavController

private val LocalController = compositionLocalOf<Controller?> { null }
private var LocalNavController = compositionLocalOf<NavHostController?> { null }
private var LocalIsDarkMode = compositionLocalOf { false }

@Composable
fun TestProvider(controller: Controller, content: () -> Unit) {
    val navController = rememberNavController()
    val isDark = isSystemInDarkTheme()

    CompositionLocalProvider(LocalController provides controller) {
        CompositionLocalProvider(LocalNavController provides navController) {
            CompositionLocalProvider(LocalIsDarkMode provides isDark) { }
            content()
        }
    }
}

@Composable
fun ProductionProvider(controller: Controller = Controller.fromConstants(), content: @Composable () -> Unit) {
    val navController = rememberNavController()
    var isLoading by remember { mutableStateOf(controller.bridge.onDidLoad.getLastValue()?.let { false } ?: true) }
    var hasInternet by remember { mutableStateOf(controller.bridge.onHasInternet.getLastValue() ?: true) }
    var error by remember { mutableStateOf(controller.bridge.onError.getLastValue()) }

    val modifier = Modifier
        .fillMaxWidth()
        .fillMaxHeight()
        .background(
            if (LocalIsDarkMode.current) {
                controller.darkBackgroundColor
            } else {
                controller.lightBackgroundColor
            }
        )
    val textColor = if (LocalIsDarkMode.current) { controller.darkForegroundColor } else { controller.lightForegroundColor }

    LaunchedEffect(Unit) {
        controller.bridge.start(controller.wsUrl)

        controller.bridge.onDidLoad.listen(ListenId()) {
            isLoading = false
        }

        controller.bridge.onHasInternet.listen(ListenId()) {
            hasInternet = it
        }

        controller.bridge.onHasInternet.listen(ListenId()) {
            hasInternet = true
        }

        controller.bridge.onError.listen(ListenId()) {
            error = it
        }
    }

    Box {
        Box(modifier.zIndex(1f)) {
            CompositionLocalProvider(LocalController provides controller) {
                CompositionLocalProvider(LocalNavController provides navController) {
                    content()
                }
            }
        }
        if (isLoading) {
            Box(modifier.zIndex(2f)) {
                Column {
                    Text("Loading...", fontSize = 20.sp, color = textColor)
                }
            }
        }
        if (!hasInternet) {
            Box(modifier.zIndex(3f)) {
                Column {
                    Text(controller.noInternetHeader, fontSize = 30.sp, color = textColor)
                    Text(controller.noInternetContent, color = textColor)
                }
            }
        }
        if (error != null) {
            Box(modifier.zIndex(3f)) {
                Column {
                    Text(controller.errorHeader, fontSize = 30.sp, color = textColor)
                    Text(error!!, color = textColor)
                }
            }
        }
    }
}

@Composable
fun useTheme(): State<Theme> {
    val controller = LocalController.current!!
    val theme = remember { mutableStateOf(controller.bridge.onThemeChanged.getLastValue() ?: Theme.testDefault()) }

    DisposableEffect(Unit) {
        val listenId = ListenId()
        controller.bridge.onThemeChanged.listen(listenId) { newTheme -> theme.value = newTheme }
        theme.value = controller.bridge.onThemeChanged.getLastValue() ?: Theme.testDefault()

        onDispose {
            controller.bridge.onThemeChanged.removeListener(listenId)
        }
    }

    return theme
}

@Composable
fun useObjects(ids: List<String>): List<Pair<String, Object>> {
    val controller = LocalController.current!!
    var objects by remember { mutableStateOf(controller.objectStore.getCurrentObjects(ids)) }

    DisposableEffect(ids) {
        val listenId = ListenId()
        controller.objectStore.listen(listenId, ids) { newObjects ->
            objects = newObjects
        }
        objects = controller.objectStore.getCurrentObjects(ids)

        onDispose {
            controller.objectStore.removeListener(listenId)
        }
    }

    return objects
}

@Composable
fun useObject(id: String?): State<Object?> {
    val objects = useObjects(
        if (id != null) {
            listOf(id)
        } else {
            listOf()
        }
    )

    return remember { derivedStateOf { objects.firstOrNull()?.second } }
}

@Composable
fun useNavController(): NavHostController {
    val navController = LocalNavController.current
        ?: throw Exception("useNavController can only be used in a child of Provider")

    return navController
}

@Composable
fun useSurface(surface: String? = null): State<SurfaceTheme> {
    val theme = useTheme()
    val isDark = LocalIsDarkMode.current

    return remember {
        derivedStateOf {
            val customSurface = if (surface != null) {
                if (isDark) {
                    theme.value.darkSurfaces[surface]
                } else {
                    theme.value.lightSurfaces[surface]
                }
            } else {
                null
            }

            customSurface
                ?: if (isDark) {
                    theme.value.defaultDarkSurface
                } else {
                    theme.value.defaultLightSurface
                }
        }
    }
}
