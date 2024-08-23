import SwiftUI

class ComponentModel: ObservableObject, Identifiable {
    var id: String?
    
    @Published var label: LabelModel?
    @Published var flex: FlexModel?
    
    func update(data: Any) {
        guard let object = data as? [String:Any] else {
            print("Not an object")
            return
        }
        
        guard let type = object["type"] as? String else {
            print("Missing type")
            return
        }
        
        guard let def = object["def"] else {
            print("Missing def")
            return
        }
        
        guard let defObject = def as? [String: Any] else {
            print("Bad object")
            return
        }
        
        if let id = defObject["_updateId"] as? String {
            self.id = id
        }
        
        if type == "Label" {
            self.label = LabelModel()
            self.label?.update(data: defObject)
        } else if type == "Flex"{
            self.flex = FlexModel()
            self.flex?.update(data: defObject)
        }
    }
}

private struct ComponentRender: View {
    @ObservedObject var model: ComponentModel
    
    var body: some View {
        if let model = model.label {
            Label(model: model)
        }
        
        if let model = model.flex {
            Flex(model: model)
        }
    }
}

struct Component: View {
    @ObservedObject var model: ComponentModel
    
    var body: some View {
        if let id = self.model.id {
            ComponentRender(model: model)
                .onAppear {
                    Bridge.shared.onUpdate(id) { state in
                        self.model.update(data: state)
                    }
                }
                .onDisappear {
                    Bridge.shared.removeListener(id)
                }
        } else {
            ComponentRender(model: model)
        }
    }
}
