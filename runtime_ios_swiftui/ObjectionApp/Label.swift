import SwiftUI

class LabelModel: ObservableObject {
    @Published var text = "nil"
    
    func update(data: [String: Any]) {
        if let text = data["text"] as? String {
            self.text = text
            print("set text \(self.text)")
        } else {
            print("nothing to update")
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
    Label()
}
