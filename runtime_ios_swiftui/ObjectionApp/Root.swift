import SwiftUI

struct Root: View {
    @StateObject private var model = ComponentModel();
    @State private var isLoading = true
    @State private var error: String?
    @State private var hasNoInternet = false
    
    var body: some View {
        VStack {
            if let error = error {
                VStack(spacing: 20) {
                    Text(error)
                    if !isLoading {
                        Button(action: { self.error = nil }) {
                            Text("Continue")
                        }
                    }
                }.background()
            } else if isLoading {
                Text("Loading...")
                .onAppear {
                    Bridge.shared.onInitial = { state in
                        self.isLoading = false
                        self.model.update(data: state)
                    }
                    
                    Bridge.shared.onError = { message in
                        self.error = message
                    }
                    
                    Bridge.shared.start(url: "ws://localhost:8000")
                }
            }
            
            Component(model: model)
        }
    }
}

#Preview {
    Root()
}
