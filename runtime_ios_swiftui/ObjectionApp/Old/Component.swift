//import SwiftUI
//
//private enum ComponentInner {
//    case label(LabelModel)
//    case container(ContainerModel)
//    case space(SpaceModel)
//    case divider(DividerModel)
//    case image(ImageModel)
//    case fragment
//}
//
//class ComponentModel: ObservableObject, Identifiable {
//    var id = UUID()
//    var updateId: String?
//    
//    @Published fileprivate var inner: ComponentInner = .fragment
//    
//    func update(data: Any) {
//        guard let object = data as? [String:Any] else {
//            print("Not an object")
//            return
//        }
//        
//        guard let type = object["type"] as? String else {
//            print("Missing type")
//            return
//        }
//        
//        guard let def = object["def"] else {
//            print("Missing def")
//            return
//        }
//        
//        guard let defObject = def as? [String: Any] else {
//            print("Bad object")
//            return
//        }
//        
//        if let id = defObject["_update_id"] as? String {
//            self.updateId = id
//        }
//        
//        if type == "Label" {
//            if case .label(let model) = self.inner {
//                model.update(data: defObject)
//            } else {
//                let model = LabelModel()
//                model.update(data: defObject)
//                
//                self.inner = .label(model)
//            }
//        } else if type == "Container" {
//            if case .container(let containerModel) = inner {
//                containerModel.update(data: defObject)
//            } else {
//                let model = ContainerModel()
//                model.update(data: defObject)
//                
//                self.inner = .container(model)
//            }
//        } else if type == "Space" {
//            if case .space(let spaceModel) = inner {
//                spaceModel.update(data: defObject)
//            } else {
//                let model = SpaceModel()
//                model.update(data: defObject)
//                
//                self.inner = .space(model)
//            }
//        } else if type == "Divider" {
//            if case .divider(let dividerModel) = inner {
//                dividerModel.update(data: defObject)
//            } else {
//                let model = DividerModel()
//                model.update(data: defObject)
//                
//                self.inner = .divider(model)
//            }
//        } else if type == "Image" {
//            if case .image(let imageModel) = inner {
//                imageModel.update(data: defObject)
//            } else {
//                let model = ImageModel()
//                model.update(data: defObject)
//                
//                self.inner = .image(model)
//            }
//        } else if type == "Fragment" {} else {
//            print("Unknown model")
//        }
//    }
//}
//
//private struct ComponentRender: View {
//    @ObservedObject var model: ComponentModel
//    
//    var body: some View {
//        if case .label(let model) = model.inner {
//            Label(model: model)
//        } else if case .container(let model) = model.inner {
//            Container(model: model)
//        } else if case .space(let model) = model.inner {
//            Space(model: model)
//        } else if case .divider(let model) = model.inner {
//            Divider(model: model)
//        } else if case .image(let model) = model.inner {
//            Image(model: model)
//        } else if case .fragment = model.inner {
//            EmptyView()
//        } else {
//            let _ = print("Unknown component")
//        }
//    }
//}
//
//struct Component: View {
//    @ObservedObject var model: ComponentModel
//    
//    var body: some View {
//        if let id = self.model.updateId {
//            ComponentRender(model: model)
//                .onAppear {
//                    Bridge.shared.onUpdate(id) { state in
//                        self.model.update(data: state)
//                    }
//                }
//                .onDisappear {
//                    Bridge.shared.removeListener(id)
//                }
//        } else {
//            ComponentRender(model: model)
//        }
//    }
//}
//
