package com.example.objectionapp

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SearchBar
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier

@Composable
fun Layout() {
    ThemeProvider { theme ->
        val (selectedObjectId, setSelectedObjectId) = remember { mutableStateOf(theme.getInitialObjectId()) }

        MaybeObjectProvider(theme.getInitialObjectId()) { obj ->
            Scaffold(
                bottomBar = {
                    if (theme.tabBar != null && selectedObjectId != null) {
                        TabBarView(theme.tabBar, selectedObjectId) { newObjectId -> setSelectedObjectId(newObjectId) }
                    }
                },
                content = { padding ->
                    Text("Hello, World!", modifier = Modifier.padding(padding))
                }
            )
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TabBarView(tabBar: TabBar, selectedObjectId: String, onNavigation: (String) -> Unit) {
    ObjectsProvider(tabBar.objects) { objects ->
        for ((id, obj) in objects) {
            NavigationBar {
                SearchBar(
                    query = "Hoo",
                    onQueryChange = {},
                    active = false,
                    content = {},
                    onSearch = {},
                    onActiveChange = {}
                )

                NavigationBarItem(
                    selected = selectedObjectId == id,
                    icon = { StandardIcon(obj.icon ?: "Help") },
                    onClick = { onNavigation(id) }
                )
            }
        }
    }
}