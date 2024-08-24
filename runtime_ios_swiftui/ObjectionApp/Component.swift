import SwiftUI

private enum ComponentInner {
    case label(LabelModel)
    case container(ContainerModel)
    case fragment
}

class ComponentModel: ObservableObject, Identifiable {
    var id = UUID()
    var updateId: String?
    
    @Published fileprivate var inner: ComponentInner = .fragment
    
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
        
        if let id = defObject["_update_id"] as? String {
            self.updateId = id
        }
        
        if type == "Label" {
            if case .label(let model) = self.inner {
                model.update(data: defObject)
            } else {
                let model = LabelModel()
                model.update(data: defObject)
                
                self.inner = .label(model)
            }
        } else if type == "Container" {
            if case .container(let containerModel) = inner {
                containerModel.update(data: defObject)
            } else {
                let model = ContainerModel()
                model.update(data: defObject)
                
                self.inner = .container(model)
            }
        }
    }
}

private struct ComponentRender: View {
    @ObservedObject var model: ComponentModel
    
    var body: some View {
        if case .label(let model) = model.inner {
            Label(model: model)
        }
        
        if case .container(let model) = model.inner {
            Container(model: model)
        }
    }
}

struct Component: View {
    @ObservedObject var model: ComponentModel
    
    var body: some View {
        if let id = self.model.updateId {
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

