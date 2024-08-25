import SwiftUI

class DividerModel: ObservableObject {
    @Published var thickness: Double?
    @Published var color: Color?
    
    func update(data: [String: Any]) {
        if let thickness = data["thickness"] {
            self.thickness = buildDouble(thickness)
        }
        
        if let color = data["color"] {
            self.color = buildColor(color)
        }
    }
}

struct Divider: View {
    @ObservedObject var model: DividerModel
    
    var body: some View {
        SwiftUI.Divider().frame(
            minWidth: model.thickness ?? 0.0,
            minHeight: model.thickness ?? 0.0
        )
        .background(model.color ?? Color.clear)
    }
}

#Preview {
    let model = ComponentModel()
    model.update(data: [
        "type": "Container",
        "def": [
            "alignment": "Start",
            "children": [
                [ "type": "Label", "def": [ "text": "Hello, World!" ] ],
                [ "type": "Divider", "def": [ "thickness": 2, "color": [200, 200, 200, 255] ] ],
                [ "type": "Label", "def": [ "text": "Foo" ] ]
            ]
        ]
    ])

    return Component(model: model)
}
