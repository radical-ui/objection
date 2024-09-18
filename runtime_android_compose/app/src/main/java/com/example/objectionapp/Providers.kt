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

@Composable
fun Provider(controller: Controller = Controller.fromConstants(), content: @Composable () -> Unit) {
	val navController = rememberNavController()
	val isLoading = remember {
		mutableStateOf(controller.bridge.onDidLoad.getLastValue()?.let { false } ?: true)
	}
	val hasInternet =
		remember { mutableStateOf(controller.bridge.onHasInternet.getLastValue() ?: true) }
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
	val textColor = if (isDark) {
		controller.darkForegroundColor
	} else {
		controller.lightForegroundColor
	}

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
fun useNavController(): NavHostController {
	val navController = LocalNavController.current
		?: throw Exception("useNavController can only be used in a child of Provider")

	return navController
}
