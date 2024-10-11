import SwiftUI

struct Root: View {
    @Environment(GlobalTheme.self) private var globalTheme
    
    var body: some View {
        if let tabBar = globalTheme.theme.tabBar {
            TabBarView(bar: tabBar)
        }
    }
}

struct TabBarView: View {
    let bar: TabBar
    @State private var selectedIndex = 0
    
    var body: some View {
        ObjectsProvider(bar.objects) {objects in
            TabView(selection: $selectedIndex) {
                let _ = print(bar.objects)
                
                ForEach(Array(objects.enumerated()), id: \.element) { index, object in
                    let title = object.title ?? "untitled"
                    
                    NavigationStack {
                        ScrollView {
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                            Text("Hello")
                        }
                        .searchable(
                            text: .constant(""),
                            placement: .toolbar,
                            prompt: "Foo"
                        )
                        .navigationTitle(title)
                        .toolbar {
                            ForEach(object.actions) { action in
                                Text("TODO Action")
                            }
                        }
                    }
                    .tabItem {
                        Text(title)
                        
                        if let icon = object.icon {
                            Image(systemName: icon)
                        }
                    }
                    .tag(index)
                }
            }
        }
    }
}

#Preview {
    GlobalProvider {
        Root()
    }
}
