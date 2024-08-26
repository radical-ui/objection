import SwiftUI

struct Background: View {
    @Environment(\.colorScheme) var colorScheme
    
    var body: some View {
        Color(colorScheme == .dark ? darkBackgroundColor : lightBackgroundColor).ignoresSafeArea(.all)
    }
}

struct ErrorText: View {
    @Environment(\.colorScheme) var colorScheme
    
    let text: String
    
    var body: some View {
        Text(text).foregroundStyle(colorScheme == .dark ? darkForegroundColor : lightForegroundColor)
    }
}

struct Root: View {
    @StateObject private var model = ComponentModel();
    @State private var isLoading = true
    @State private var error: String?
    @State private var hasNoInternet = false
    
    var body: some View {
        ZStack {
            ZStack {
                Background()
                Component(model: model)
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
            Bridge.shared.onInitial = { state in
                self.isLoading = false
                self.model.update(data: state)
            }
            
            Bridge.shared.onError = { message in
                self.error = message
            }
            
            Bridge.shared.onNoInternet = {
                self.hasNoInternet = true
            }
            
            Bridge.shared.onHasInternet = {
                self.hasNoInternet = false
            }
            
            Bridge.shared.start(url: "ws://localhost:8000")
        }
    }
}

#Preview {
    Root()
}
