package com.example.objectionapp

import androidx.activity.enableEdgeToEdge
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
import androidx.compose.runtime.compositionLocalOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.sp
import androidx.compose.ui.zIndex
import androidx.navigation.NavHostController
import androidx.navigation.compose.rememberNavController
import androidx.activity.enableEdgeToEdge

private val LocalController = compositionLocalOf<Controller?> { null }
private var LocalNavController = compositionLocalOf<NavHostController?> { null }

@Composable
fun Provider(controller: Controller = Controller.fromConstants()) {

	CompositionLocalProvider(LocalController provides controller) {
		val navController = rememberNavController()
		val hasInternet = remember { mutableStateOf(controller.bridge.onHasInternet.getLastValue() ?: true) }
		val error = remember { mutableStateOf(controller.bridge.onError.getLastValue()) }
		val isDark = isSystemInDarkTheme()
		val isLoading = useObject(defaultThemeId) === null || useObject(defaultLayoutId) === null
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
		val textColor = if (isDark) {
			controller.darkForegroundColor
		} else {
			controller.lightForegroundColor
		}

		LaunchedEffect(Unit) {
			controller.bridge.start(controller.wsUrl)

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
			if (!isLoading) {
				Box(modifier.zIndex(1f)) {
					CompositionLocalProvider(LocalNavController provides navController) {
						RenderTheme {
							RenderDefaultLayout()
						}
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
}

@Composable
fun TestProvider(controller: Controller) {
	val navController = rememberNavController()

	CompositionLocalProvider(LocalController provides controller) {
		CompositionLocalProvider(LocalNavController provides navController) {
			if (useObject(defaultThemeId) !== null && useObject(defaultLayoutId) !== null) {
				RenderTheme {
					RenderDefaultLayout()
				}
			}
		}
	}
}

@Composable
fun useObject(id: String?): Object? {
	val controller = LocalController.current!!
	var obj by remember { mutableStateOf(id?.let { controller.objectStore.getCurrentObject(id) }) }

	DisposableEffect(id) {
		if (id != null) {
			val listenId = ListenId()
			controller.objectStore.listen(listenId, id) { newObj ->
				obj = newObj
			}
			obj = controller.objectStore.getCurrentObject(id)

			onDispose {
				controller.objectStore.removeListener(listenId, id)
			}
		} else {
			onDispose { }
		}
	}

	return obj
}

@Composable
fun useDefaultTheme(): Theme {
	val obj = useObject(defaultThemeId) ?: throw Exception("No object exists for '$defaultThemeId'")

	return if (obj is Object.Theme) obj.def else throw Exception("Object '$defaultThemeId' was not a theme")
}

@Composable
fun useDefaultLayout(): Layout {
	val obj = useObject(defaultLayoutId) ?: throw Exception("No object exists for '$defaultLayoutId'")

	return if (obj is Object.Layout) obj.def else throw Exception("Object '$defaultLayoutId' was not a layout")
}

@Composable
fun usePage(id: String?): Page? {
	val obj = useObject(id) ?: return null

	return if (obj is Object.Page) obj.def else throw Exception("Object '$id' was not a page")
}

@Composable
fun useNavController(): NavHostController {
	val navController = LocalNavController.current ?: throw Exception("useNavController can only be used in a child of Provider")

	return navController
}
