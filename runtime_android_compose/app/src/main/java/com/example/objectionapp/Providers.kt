package com.example.objectionapp

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.compositionLocalOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember

val LocalController = compositionLocalOf<Controller?> { null }
val LocalTheme = compositionLocalOf { Theme.testDefault() }

@Composable
fun Root(controller: Controller, content: @Composable () -> Unit) {
    DisposableEffect(true) {
        println("Start")
        controller.bridge.start(controller.wsUrl)

        onDispose {
            println("End")
        }
    }

    CompositionLocalProvider(LocalController provides controller) {
        content()
    }
}

@Composable
fun ThemeProvider(content: @Composable (Theme) -> Unit) {
    val controller = LocalController.current!!
    val (theme, setTheme) = remember { mutableStateOf(controller.bridge.getCurrentTheme()) }

    DisposableEffect(Unit) {
        val listenId = ListenId()
        controller.bridge.onThemeSet.listen(listenId) {newTheme -> setTheme(newTheme)}

        onDispose {
            controller.bridge.onThemeSet.removeListener(listenId)
        }
    }

    content(theme)
}

@Composable
fun ObjectsProvider(ids: List<String>, content: @Composable (List<Pair<String, Object>>) -> Unit) {
    val controller = LocalController.current!!
    val (objects, setObjects) = remember { mutableStateOf(controller.objectStore.getCurrentObjects(ids)) }

    DisposableEffect(Unit) {
        val listenId = ListenId()
        controller.objectStore.listen(listenId, ids) { newObjects ->
            setObjects(newObjects)
        }

        onDispose {
            controller.objectStore.removeListener(listenId)
        }
    }

    content(objects)
}

@Composable
fun ObjectProvider(id: String, content: @Composable (Object) -> Unit) {
    ObjectsProvider(listOf(id)) { objects ->
        if (objects.isNotEmpty()) {
            content(objects[0].second)
        }
    }
}

@Composable
fun MaybeObjectProvider(id: String?, content: @Composable (Object?) -> Unit) {
    ObjectsProvider(if (id != null) { listOf(id) } else { listOf() }) { objects ->
        if (objects.isNotEmpty()) {
            content(objects[0].second)
        } else {
            content(null)
        }
    }
}
