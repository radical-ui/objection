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
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.sp
import androidx.compose.ui.zIndex
import androidx.navigation.NavHostController
import androidx.navigation.compose.rememberNavController

private val LocalController = compositionLocalOf<Controller?> { null }
private var LocalNavController = compositionLocalOf<NavHostController?> { null }

@Composable
fun Provider(controller: Controller = Controller.fromConstants(), content: @Composable () -> Unit) {
    val navController = rememberNavController()
    val isLoading = remember { mutableStateOf(controller.bridge.onDidLoad.getLastValue()?.let { false } ?: true) }
    val hasInternet = remember { mutableStateOf(controller.bridge.onHasInternet.getLastValue() ?: true) }
    val error = remember { mutableStateOf<String?>(controller.bridge.onError.getLastValue()) }
    val isDark = isSystemInDarkTheme()
    val modifier = Modifier
        .fillMaxWidth()
        .fillMaxHeight()
        .background(
            if (isDark) {
                controller.darkBackgroundColor
            } else {
                controller.lightBackgroundColor
            }
        )
    val textColor = if (isDark) { controller.darkForegroundColor } else { controller.lightForegroundColor }

    LaunchedEffect(Unit) {
        controller.bridge.start(controller.wsUrl)

        controller.bridge.onDidLoad.listen(ListenId()) {
            isLoading.value = false
        }

        controller.bridge.onHasInternet.listen(ListenId()) {
            hasInternet.value = it
        }

        controller.bridge.onHasInternet.listen(ListenId()) {
            hasInternet.value = true
        }

        controller.bridge.onError.listen(ListenId()) {
            error.value = it
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
        if (isLoading.value) {
            Box(modifier.zIndex(2f)) {
                Column {
                    Text("Loading...", fontSize = 20.sp, color = textColor)
                }
            }
        }
        if (!hasInternet.value) {
            Box(modifier.zIndex(3f)) {
                Column {
                    Text(controller.noInternetHeader, fontSize = 30.sp, color = textColor)
                    Text(controller.noInternetContent, color = textColor)
                }
            }
        }
        if (error.value != null) {
            Box(modifier.zIndex(3f)) {
                Column {
                    Text(controller.errorHeader, fontSize = 30.sp, color = textColor)
                    Text(error.value!!, color = textColor)
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
fun useObjects(ids: List<String>): State<List<Pair<String, Object>>> {
    val controller = LocalController.current!!
    val objects = remember { mutableStateOf(controller.objectStore.getCurrentObjects(ids)) }

    DisposableEffect(ids) {
        val listenId = ListenId()
        controller.objectStore.listen(listenId, ids) { newObjects ->
            objects.value = newObjects
        }
        objects.value = controller.objectStore.getCurrentObjects(ids)

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

    return remember { derivedStateOf { objects.value.firstOrNull()?.second } }
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
    val isDark = isSystemInDarkTheme()

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
