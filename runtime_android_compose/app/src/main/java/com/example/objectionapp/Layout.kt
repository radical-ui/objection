package com.example.objectionapp

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
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
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.navigation.NavController
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.dialog
import androidx.navigation.compose.rememberNavController
import coil.compose.AsyncImage

@Composable
fun Layout() {
    val navController = useNavController()
    val theme = useTheme()
    val surface = useSurface()

    Scaffold(
        containerColor = surface.value.backgroundColor1.intoColor(),
        bottomBar = { BottomBarView() },
        content = { padding ->
            val initialObjectId = theme.value.getInitialObjectId()

            if (initialObjectId != null) {
                NavHost(
                    navController = navController,
                    startDestination = encodeObjectIdIntoPageRoute(initialObjectId)
                ) {
                    composable(getObjectIdPageRouteTemplate()) { navBackStackEntry ->
                        Page(
                            objectId = decodeObjectIdFromRouteArgs(navBackStackEntry.arguments),
                            modifier = Modifier.padding(bottom = padding.calculateBottomPadding())
                        )
                    }
                    dialog(getObjectIdDialogRouteTemplate()) { navBackStackEntry ->
                        Page(
                            objectId = decodeObjectIdFromRouteArgs(navBackStackEntry.arguments),
                            modifier = Modifier.padding(bottom = padding.calculateBottomPadding())
                        )
                    }
                }
            }
        }
    )
}

@Composable
private fun BottomBarView() {
    val navController = useNavController()
    val theme = useTheme()
    val surface = useSurface(theme.value.navigationSurface)
    val tabBarObjects = useObjects(theme.value.tabBar?.objects ?: listOf())

    if (tabBarObjects.value.isNotEmpty()) {
        NavigationBar(
            containerColor = surface.value.backgroundColor2.intoColor(),
            content = {
                val history = remember { mutableStateOf<List<String>>(listOf()) }

                DisposableEffect(Unit) {
                    val listener =
                        NavController.OnDestinationChangedListener { _, _, arguments ->
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
                                popUpTo(navController.graph.findStartDestination().id)
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
            }
        )
    }
//    if (currentObject.actions.isNotEmpty()) {
//        BottomAppBar(
//            actions = {
//                for (action in currentObject.actions) {
//                    IconButton(
//                        content = { StandardIcon("") },
//                        onClick = { }
//                    )
//                }
//            }
//        )
//    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun Page(objectId: String, modifier: Modifier = Modifier) {
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

    Column(modifier.background(surface.value.backgroundColor1.intoColor())) {
        val inner = @Composable { Text("${obj.value?.title}") }

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
                },
                title = inner,
                scrollBehavior = scrollBehavior,
                colors = colors
            )
        }

        LazyColumn(
            verticalArrangement = Arrangement.spacedBy(20.dp),
            modifier = Modifier
                .nestedScroll(scrollBehavior.nestedScrollConnection)
                .fillMaxWidth()
                .fillMaxHeight()
        ) {
            val modifier = Modifier.padding(horizontal = 16.dp)

            obj.value?.subtitle?.let { subtitle ->
                item {
                    Text(
                        subtitle,
                        color = surface.value.foregroundColor3.intoColor(),
                        fontSize = 18.sp,
                        modifier = modifier
                    )
                }
            }

            obj.value?.image?.let { url ->
                item {
                    AsyncImage(
                        model = url,
                        contentDescription = "An image",
                        clipToBounds = true,
                        contentScale = ContentScale.FillBounds,
                        modifier = modifier
                            .clip(RoundedCornerShape(8))
                            .height(300.dp)
                            .fillMaxWidth()
                    )
                }
            }

            obj.value?.content?.let { content ->
                for (item in content) {
                    item { ContentView(item, modifier) }
                }
            }
        }
    }
}

