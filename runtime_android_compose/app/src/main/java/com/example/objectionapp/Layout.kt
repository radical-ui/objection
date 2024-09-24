package com.example.objectionapp

import androidx.compose.animation.AnimatedVisibilityScope
import androidx.compose.animation.ExperimentalSharedTransitionApi
import androidx.compose.animation.SharedTransitionLayout
import androidx.compose.animation.SharedTransitionScope
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.absolutePadding
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.paddingFrom
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.IconButton
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemColors
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarColors
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.rememberTopAppBarState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.layout.AlignmentLine
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.layout.MeasureResult
import androidx.compose.ui.layout.layout
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.dialog
import coil.compose.AsyncImage
import kotlinx.serialization.Serializable

@Serializable
data class Layout(
	@Description(
		"The tab bar is shown at the bottom of the application. If there is no current page set, the current page will default to the first tab bar item."
	) val tabBar: TabBar? = null,

	@Description("The page that is to be shown by default") @ObjectReference(Object.Page::class) val currentPageId: String? = null,
) {
	fun getRoots(): List<String> {
		return (tabBar?.buttons?.map { it.pageId } ?: listOf()) + (currentPageId?.let { listOf(it) } ?: listOf())
	}

	fun getInitialPageId(): String? {
		return currentPageId ?: tabBar?.buttons?.first()?.pageId
	}
}

@OptIn(ExperimentalSharedTransitionApi::class)
@Composable
fun RenderDefaultLayout() {
	val navController = useNavController()
	val layout = useDefaultLayout()

	Scaffold(
		bottomBar = { layout.tabBar?.let { TabBarRender(it) } },
		content = { padding ->
			val initialObjectId = layout.getInitialPageId()

			if (initialObjectId != null) {
				SharedTransitionLayout {
					NavHost(
						navController = navController, startDestination = encodeObjectIdIntoPageRoute(initialObjectId)
					) {
						composable(getObjectIdPageRouteTemplate()) { navBackStackEntry ->
							PageRender(
								id = decodeObjectIdFromRouteArgs(navBackStackEntry.arguments),
								bottomPadding = padding.calculateBottomPadding(),
								animatedVisibilityScope = this,
							)
						}
						dialog(getObjectIdDialogRouteTemplate()) { navBackStackEntry ->
							PageRender(
								id = decodeObjectIdFromRouteArgs(navBackStackEntry.arguments),
								bottomPadding = padding.calculateBottomPadding(),
								animatedVisibilityScope = null,
							)
						}
					}
				}
			}
		})
}

@Composable
@Preview()
fun SingleLayoutTest() {
	val controller = Controller.fromConstants()
	controller.objectStore.preload("theme_default", Object.Theme(Theme()))
	controller.objectStore.preload(
		"layout_default", Object.Layout(
			Layout(
				tabBar = TabBar(
					buttons = listOf(
						TabBarButton("page1", "Search"),
						TabBarButton("page2", "Home")
					)
				)
			)
		)
	)
	controller.objectStore.preload(
		"page1", Object.Page(
			Page(title = "Page1")
		)
	)
	controller.objectStore.preload(
		"page2", Object.Page(
			Page(title = "Page2")
		)
	)

	TestProvider(controller)
}
