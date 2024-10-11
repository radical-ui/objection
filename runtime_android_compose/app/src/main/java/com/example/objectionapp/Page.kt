package com.example.objectionapp

import androidx.compose.animation.AnimatedVisibilityScope
import androidx.compose.animation.ExperimentalSharedTransitionApi
import androidx.compose.animation.SharedTransitionScope
import androidx.compose.animation.core.tween
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.IconButton
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.rememberTopAppBarState
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import kotlinx.serialization.Serializable

@Serializable
data class Page(
	@Description("The page title will be displayed prominently at the top of the screen") val title: String? = null,
	@Description("The images will be displayed in carousel form, directly below the title") val imageUrls: List<String>? = null,
	@Description("The page subtitle is displayed directly under any images on the page") val subtitle: String? = null,

	@Description(
		"The page that will be pulled up for a presumed search through the contents of this page"
	) @ObjectReference(Object.Page::class) val searchPage: String? = null,

	// TODO: refine these
	val content: List<Content> = listOf(),
	val actions: List<Action> = listOf(),
)

@OptIn(ExperimentalMaterial3Api::class, ExperimentalSharedTransitionApi::class)
@Composable
fun SharedTransitionScope.PageRender(
	id: String, bottomPadding: Dp, animatedVisibilityScope: AnimatedVisibilityScope?
) {
	val navController = useNavController()
	val layout = useDefaultLayout()
	val isRoot = layout.getRoots().contains(id)
	val page = usePage(id)

	val scrollBehavior = if (isRoot) {
		TopAppBarDefaults.exitUntilCollapsedScrollBehavior(rememberTopAppBarState())
	} else {
		TopAppBarDefaults.pinnedScrollBehavior(rememberTopAppBarState())
	}

	Column(
		Modifier.padding(bottom = bottomPadding)
	) {
		if (isRoot) {
			LargeTopAppBar(
				title = { Text("${page?.title}") },
				scrollBehavior = scrollBehavior,
			)
		} else {
			TopAppBar(
				navigationIcon = {
					IconButton(onClick = { navController.popBackStack() }) {
						StandardIcon("ArrowBack")
					}
				},
				title = { Text("${page?.title}") },
				scrollBehavior = scrollBehavior,
			)
		}

		LazyColumn(
			verticalArrangement = Arrangement.spacedBy(20.dp),
			modifier = Modifier
				.nestedScroll(scrollBehavior.nestedScrollConnection)
				.fillMaxWidth()
				.fillMaxHeight()
		) {
			val childPadding = PaddingValues(horizontal = 16.dp)

			// TODO support multiple images, but keep in mind that probably only the first one should take part in
			//  the shared element animation
			page?.imageUrls?.first()?.let { url ->
				item {
					AsyncImage(
						model = url,
						contentDescription = "An image",
						clipToBounds = true,
						contentScale = ContentScale.Crop,
						modifier = if (animatedVisibilityScope != null) {
							Modifier.sharedElement(state = rememberSharedContentState("${id}/image"),
								animatedVisibilityScope = animatedVisibilityScope,
								boundsTransform = { _, _ ->
									tween(durationMillis = 300)
								})
						} else {
							Modifier
						}
							.padding(childPadding)
							.clip(RoundedCornerShape(8))
							.height(300.dp)
							.fillMaxWidth()
					)
				}
			}

			page?.content?.let { content ->
				for (item in content) {
					item { ContentView(item, childPadding, animatedVisibilityScope) }
				}
			}

			item {
				Box(Modifier.padding(vertical = 8.dp))
			}
		}
	}
}