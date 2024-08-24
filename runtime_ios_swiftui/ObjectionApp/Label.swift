import SwiftUI

class LabelModel: ObservableObject {
    @Published var text = ""

    func update(data: [String: Any]) {
        if let text = data["text"] as? String {
            self.text = text
        } else {
            print("WARN: Label with nothing to update")
        }
    }
}

struct Label: View {
    @ObservedObject var model = LabelModel()
    
    var body: some View {
        Text(model.text)
    }
}

#Preview {
    var model = LabelModel()
    
    return Label(model: model).onAppear {
        model.update(data: [
            "text": "Hello, World!"
        ])
    }
}
