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
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.dialog
import coil.compose.AsyncImage

@OptIn(ExperimentalSharedTransitionApi::class)
@Composable
fun Layout() {
    val navController = useNavController()
    val theme = useTheme()
    val surface = useSurface()

    Scaffold(containerColor = surface.value.backgroundColor1.intoColor(),
        bottomBar = { BottomBarView() },
        content = { padding ->
            val initialObjectId = theme.value.getInitialObjectId()

            if (initialObjectId != null) {
                SharedTransitionLayout {
                    NavHost(
                        navController = navController,
                        startDestination = encodeObjectIdIntoPageRoute(initialObjectId)
                    ) {
                        composable(getObjectIdPageRouteTemplate()) { navBackStackEntry ->
                            Page(
                                objectId = decodeObjectIdFromRouteArgs(navBackStackEntry.arguments),
                                bottomPadding = padding.calculateBottomPadding(),
                                animatedVisibilityScope = this,
                            )
                        }
                        dialog(getObjectIdDialogRouteTemplate()) { navBackStackEntry ->
                            Page(
                                objectId = decodeObjectIdFromRouteArgs(navBackStackEntry.arguments),
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
private fun BottomBarView() {
    val navController = useNavController()
    val theme = useTheme()
    val surface = useSurface(theme.value.navigationSurface)
    val tabBarObjects = useObjects(theme.value.tabBar?.objects ?: listOf())
    val currentBackStackEntry = navController.currentBackStackEntryAsState()
    val currentObjectId = currentBackStackEntry.value?.arguments?.let { decodeObjectIdFromRouteArgs(it) }
    val currentObject = useObject(currentObjectId)

    if (tabBarObjects.value.isNotEmpty()) {
        Column {
            currentObject.value?.searchObject?.let {searchObjectId ->
                val searchObject = useObject(searchObjectId)

                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .background(surface.value.backgroundColor2.intoColor())
                        .padding(horizontal = 16.dp)
                        .padding(top = 16.dp)
                ) {
                    Box(
                        Modifier
                            .fillMaxWidth()
                            .clip(RoundedCornerShape(50))
                            .background(surface.value.backgroundColor3.intoColor())
                            .padding(vertical = 8.dp, horizontal = 16.dp)
                    ) {
                        Row(verticalAlignment = Alignment.CenterVertically, horizontalArrangement = Arrangement.spacedBy(10.dp)) {
                            searchObject.value?.icon?.let {
                                StandardIcon(it, modifier = Modifier.size(30.dp), tint = surface.value.foregroundColor4.intoColor())
                            }

                            Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
                                searchObject.value?.title?.let {
                                    Text(
                                        it,
                                        color = surface.value.foregroundColor2.intoColor(),
                                        fontSize = 16.sp,
                                        fontWeight = FontWeight.Bold
                                    )
                                }

                                searchObject.value?.subtitle?.let {
                                    Text(
                                        it,
                                        color = surface.value.foregroundColor3.intoColor(),
                                        fontSize = 14.sp,
                                    )
                                }
                            }
                        }
                    }
                }
            }

            NavigationBar(containerColor = surface.value.backgroundColor2.intoColor(), content = {
                val history = remember { mutableStateOf<List<String>>(listOf()) }

                DisposableEffect(Unit) {
                    val listener = NavController.OnDestinationChangedListener { _, _, arguments ->
                        history.value += listOf(decodeObjectIdFromRouteArgs(arguments))
                    }

                    navController.addOnDestinationChangedListener(listener)

                    onDispose {
                        navController.removeOnDestinationChangedListener(listener)
                    }
                }

                for ((id, obj) in tabBarObjects.value) {
                    NavigationBarItem(
                        selected = history.value.contains(id),
                        icon = { StandardIcon(obj.icon ?: "Help") },
                        onClick = {
                            history.value = listOf(id)
                            navController.navigate(route = encodeObjectIdIntoPageRoute(id)) {
                                popUpTo(id)
                                launchSingleTop = true
                            }
                        },
                        label = { Text("${obj.title}") },
                        colors = NavigationBarItemColors(
                            selectedIconColor = surface.value.foregroundColor1.intoColor(),
                            selectedTextColor = surface.value.foregroundColor1.intoColor(),
                            selectedIndicatorColor = surface.value.primaryColor4.intoColor(),

                            unselectedIconColor = surface.value.foregroundColor2.intoColor(),
                            unselectedTextColor = surface.value.foregroundColor2.intoColor(),

                            disabledIconColor = surface.value.foregroundColor3.intoColor(),
                            disabledTextColor = surface.value.foregroundColor3.intoColor()
                        )
                    )
                }
            })
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class, ExperimentalSharedTransitionApi::class)
@Composable
private fun SharedTransitionScope.Page(
    objectId: String, bottomPadding: Dp, animatedVisibilityScope: AnimatedVisibilityScope?
) {
    val navController = useNavController()
    val theme = useTheme()
    val surface = useSurface(theme.value.navigationSurface)
    val isRoot = theme.value.getRoots().contains(objectId)
    val obj = useObject(objectId)

    val scrollBehavior = if (isRoot) {
        TopAppBarDefaults.exitUntilCollapsedScrollBehavior(rememberTopAppBarState())
    } else {
        TopAppBarDefaults.pinnedScrollBehavior(rememberTopAppBarState())
    }

    val colors = TopAppBarColors(
        containerColor = surface.value.backgroundColor1.intoColor(),
        scrolledContainerColor = surface.value.backgroundColor2.intoColor(),
        navigationIconContentColor = surface.value.foregroundColor2.intoColor(),
        titleContentColor = surface.value.foregroundColor1.intoColor(),
        actionIconContentColor = surface.value.foregroundColor2.intoColor()
    )

    Column(
        Modifier
            .padding(bottom = bottomPadding)
            .background(surface.value.backgroundColor1.intoColor())
    ) {
        val inner = @Composable {
            obj.value?.engagedTitle?.let {
                Text(
                    it,
                    color = surface.value.foregroundColor1.intoColor(),
                    fontSize = 30.sp,
                    fontWeight = FontWeight.Bold
                )
            } ?: Text("${obj.value?.title}")
        }

        if (isRoot) {
            LargeTopAppBar(
                title = inner,
                scrollBehavior = scrollBehavior,
                colors = colors,
            )
        } else {
            TopAppBar(
                navigationIcon = {
                    IconButton(onClick = { navController.popBackStack() }) {
                        StandardIcon("ArrowBack")
                    }
                }, title = inner, scrollBehavior = scrollBehavior, colors = colors
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

            obj.value?.subtitle?.let { subtitle ->
                item {
                    Text(
                        subtitle,
                        color = surface.value.foregroundColor3.intoColor(),
                        fontSize = 18.sp,
                        modifier = Modifier.padding(childPadding)
                    )
                }
            }

            obj.value?.image?.let { url ->
                item {
                    AsyncImage(
                        model = url,
                        contentDescription = "An image",
                        clipToBounds = true,
                        contentScale = ContentScale.Crop,
                        modifier = if (animatedVisibilityScope != null) {
                            Modifier.sharedElement(state = rememberSharedContentState("${objectId}/image"),
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

            obj.value?.content?.let { content ->
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
