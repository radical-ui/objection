import SwiftUI

class FlexModel: ObservableObject {
    @Published var text = "nil"
    @Published var children: [ComponentModel] = []
    
    func update(data: [String: Any]) {
        if let text = data["text"] as? String {
            self.text = text
            print("set text \(text)")
        }
        
        if let children = data["children"] as? [Any] {
            print("children")
            self.children = children.map {
                let model = ComponentModel()
                model.update(data: $0)
                
                return model
            }
        }
    }
}

struct Flex: View {
    @ObservedObject var model: FlexModel
    
    var body: some View {
        VStack {
            Text(model.text)
            HStack {
                ForEach(model.children) { child in
                    Component(model: child)
                }
            }
        }
    }
}

#Preview {
    @State var model = FlexModel()
    
    return Flex(model: model).onAppear {
        model.update(data: [
            "text": "bar",
            "children": [
                [ "type": "Label", "def": [ "text": "Elijah" ] ]
            ]
        ])
    }
}
