import SwiftUI

class SpaceModel: ObservableObject {
    func update(data: [String: Any]) {
        // a space doesn't need any props, currently
    }
}

struct Space: View {
    @ObservedObject var model: SpaceModel
    
    var body: some View {
        Spacer()
    }
}

#Preview {
    let model = ComponentModel()
    model.update(data: [
        "type": "Container",
        "def": [
            "children": [
                [ "type": "Label", "def": [ "text": "Hello" ] ],
                [ "type": "Space", "def": [:] ],
                [ "type": "Label", "def": [ "text": "World" ] ]
            ]
        ]
    ])
    
    return Component(model: model)
}
