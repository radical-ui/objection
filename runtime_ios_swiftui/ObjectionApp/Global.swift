import SwiftUI

private var globalBridge = Bridge()
private var globalObjectStore = ObjectStore(bridge: globalBridge)

private struct Background: View {
    @Environment(\.colorScheme) var colorScheme

    var body: some View {
        Color(colorScheme == .dark ? darkBackgroundColor : lightBackgroundColor).ignoresSafeArea(.all)
    }
}

private struct ErrorText: View {
    @Environment(\.colorScheme) var colorScheme

    let text: String

    var body: some View {
        Text(text).foregroundStyle(colorScheme == .dark ? darkForegroundColor : lightForegroundColor)
    }
}

class GlobalTheme: ObservableObject {
    var theme: Theme?
    
    func startObserving() {
        // note how we are setting the event on globalObjectStore. If we were to set the event on globalBridge, globalObjectStore's event
        // would be overritten, which could result in objects being cleaned up too aggressively
        
        globalObjectStore.onThemeChanged = { theme in
            self.theme = theme
        }
    }
}

struct GlobalProvider<Content: View>: View {
    @ViewBuilder let content: () -> Content

    @StateObject private var state = GlobalTheme()
    @State private var isLoading = true
    @State private var error: String?
    @State private var hasNoInternet = false

    var body: some View {
        ZStack {
            ZStack {
                Background()
                content()
            }
            .zIndex(1)

            if isLoading {
                ZStack {
                    Background()
                    SwiftUI.Image("launch_logo")
                }
                .ignoresSafeArea(.all)
                .transition(.opacity)
                .zIndex(2)
            }

            if hasNoInternet {
                ZStack {
                    Background()

                    VStack (spacing: 20) {
                        SwiftUI.Image("no_internet")
                        ErrorText(text: noInternetHeader).font(.title)
                        ErrorText(text: noInternetContent).multilineTextAlignment(.center)
                    }
                }
                .transition(.opacity)
                .zIndex(3)
            }

            if let error = error {
                ZStack {
                    Background()

                    VStack (spacing: 30) {
                        SwiftUI.Image("error")
                        ErrorText(text: error)
                    }
                }
                .transition(.opacity)
                .zIndex(4)
            }
        }
        .onAppear {
            // there is some caution necessary here... we are only setting callbacks for the events that the object store has not already set
            // events for. This is so that they are not overridden.
            
            globalBridge.onError = { message in
                self.error = message
            }

            globalBridge.onNoInternet = {
                self.hasNoInternet = true
            }

            globalBridge.onHasInternet = {
                self.hasNoInternet = false
            }

            globalBridge.start(url: wsUrl)
        }
        .environmentObject(state)
    }
}

struct ObjectProvider<Content: View>: View {
    @ViewBuilder private var content: ([Object]) -> Content
    private var listenId = UUID()
    private var path: String
    
    init(_ path: String, @ViewBuilder content: @escaping ([Object]) -> Content) {
        self.content = content
        self.path = path
    }

    @State private var hasDoneFirstSetting = false
    @State private var objects = [Object]()
    
    var body: some View {
        Rectangle()
            .hidden()
            .onAppear {
                globalObjectStore.listen(listen_id: self.listenId, path: self.path, callback: { objects in
                    self.objects = objects
                    self.hasDoneFirstSetting = true
                })
            }
            .onDisappear {
                globalObjectStore.removeListener(listen_id: self.listenId)
            }
        
        if self.hasDoneFirstSetting {
            content(objects)
        }
    }
}
