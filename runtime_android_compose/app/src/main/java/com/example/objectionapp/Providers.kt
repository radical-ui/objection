package com.example.objectionapp

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.State
import androidx.compose.runtime.compositionLocalOf
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.navigation.NavController
import androidx.navigation.NavHostController
import androidx.navigation.compose.rememberNavController

private val LocalController = compositionLocalOf<Controller?> { null }
private var LocalNavController = compositionLocalOf<NavHostController?> { null }

@Composable
fun Provider(controller: Controller, content: @Composable () -> Unit) {
    val navController = rememberNavController()

    LaunchedEffect(Unit) {
        controller.bridge.start(controller.wsUrl)
    }

    CompositionLocalProvider(LocalController provides controller) {
        CompositionLocalProvider(LocalNavController provides navController) {
            content()
        }
    }
}

@Composable
fun useTheme(): State<Theme> {
    val controller = LocalController.current!!
    val theme = remember { mutableStateOf(controller.bridge.getCurrentTheme()) }

    DisposableEffect(Unit) {
        val listenId = ListenId()
        controller.bridge.onThemeSet.listen(listenId) { newTheme -> theme.value = newTheme }
        theme.value = controller.bridge.getCurrentTheme()

        onDispose {
            controller.bridge.onThemeSet.removeListener(listenId)
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
        controller.objectStore.listen(listenId, ids) { newObjects -> objects.value = newObjects }
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
