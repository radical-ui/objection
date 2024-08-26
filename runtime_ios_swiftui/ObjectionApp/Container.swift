import SwiftUI

enum Alignment {
    case leading, center, trailing
}

class ContainerModel: ObservableObject {
    @Published var children: [ComponentModel] = []
    @Published var spacing: Double?
    
    @Published var isHorizontal = false
    @Published var horizontalAlignment = HorizontalAlignment.center
    @Published var verticalAlignment = VerticalAlignment.center
    
    @Published var color: Color?
    
    @Published var disregardSafeArea = false
    
    @Published var paddingTop: Double?
    @Published var paddingBottom: Double?
    @Published var paddingRight: Double?
    @Published var paddingLeft: Double?
    
    @Published var cornerRadius: Double?
    
    @Published var shadowColor: Color?
    @Published var shadowX: Double?
    @Published var shadowY: Double?
    @Published var shadowRadius: Double?
    
    @Published var borderColor: Color?
    @Published var borderWidth: Double?

    func update(data: [String: Any]) {
        if let children = data["children"] as? [Any] {
            self.children = children.map { child in
                let model = ComponentModel()
                model.update(data: child)
                
                return model
            }
        }
        
        if let spacing = data["spacing"] {
            self.spacing = buildDouble(spacing)
        }
        
        if let disregardSafeArea = data["disregard_safe_area"] as? Bool {
            self.disregardSafeArea = disregardSafeArea
        }
        
        if let isHorizontal = data["is_horizontal"] as? Bool {
            self.isHorizontal = isHorizontal
        }
        
        if let alignment = data["alignment"] as? String {
            if isHorizontal {
                if alignment == "Start" { self.verticalAlignment = .top }
                else if alignment == "Center" { self.verticalAlignment = .center }
                else if alignment == "End" { self.verticalAlignment = .bottom }
            } else {
                if alignment == "Start" { self.horizontalAlignment = .leading }
                else if alignment == "Center" { self.horizontalAlignment = .center }
                else if alignment == "End" { self.horizontalAlignment = .trailing }
            }
        }
        
        if let color = data["color"] {
            self.color = buildColor(color)
        }
        
        if let padding = data["padding_top"] {
            self.paddingTop = buildDouble(padding)
        }
        if let padding = data["padding_bottom"] {
            self.paddingBottom = buildDouble(padding)
        }
        if let padding = data["padding_right"] {
            self.paddingRight = buildDouble(padding)
        }
        if let padding = data["padding_left"] {
            self.paddingLeft = buildDouble(padding)
        }
        
        if let cornerRadius = data["corner_radius"] {
            self.cornerRadius = buildDouble(cornerRadius)
        }
        
        if let shadowRadius = data["shadow_radius"] {
            self.shadowRadius = buildDouble(shadowRadius)
        }
        if let shadowX = data["shadow_x"] {
            self.shadowX = buildDouble(shadowX)
        }
        if let shadowY = data["shadow_y"] {
            self.shadowY = buildDouble(shadowY)
        }
        if let shadowColor = data["shadow_color"] {
            self.shadowColor = buildColor(shadowColor)
        }
        
        if let borderColor = data["border_color"] {
            self.borderColor = buildColor(borderColor)
        }
        if let borderWidth = data["border_width"] {
            self.borderWidth = buildDouble(borderWidth)
        }
    }
}

private struct ContainerNotDecorated: View {
    @ObservedObject var model: ContainerModel
    
    var body: some View {
        if model.isHorizontal {
            HStack(alignment: model.verticalAlignment, spacing: model.spacing ?? 0.0) {
                ForEach(model.children) { child in
                    Component(model: child)
                }
            }
        } else {
            VStack(alignment: model.horizontalAlignment, spacing: model.spacing ?? 0.0) {
                ForEach(model.children) { child in
                    Component(model: child)
                }
            }
        }
    }
}

struct Container: View {
    @ObservedObject var model: ContainerModel
    
    var body: some View {
        Group {
            Group {
                Group {
                    if model.isHorizontal {
                        HStack(alignment: model.verticalAlignment, spacing: model.spacing ?? 0.0) {
                            ForEach(model.children) { child in
                                Component(model: child)
                            }
                        }
                    } else {
                        VStack(alignment: model.horizontalAlignment, spacing: model.spacing ?? 0.0) {
                            ForEach(model.children) { child in
                                Component(model: child)
                            }
                        }
                    }
                }
                .padding(EdgeInsets(
                    top: model.paddingTop ?? 0.0,
                    leading: model.paddingLeft ?? 0.0,
                    bottom: model.paddingBottom ?? 0.0,
                    trailing: model.paddingRight ?? 0.0
                ))
                .background(model.color)
            }
            .padding(model.borderWidth ?? 0.0)
            .overlay(
                RoundedRectangle(cornerRadius: model.cornerRadius ?? 0.0)
                    .strokeBorder(model.borderColor ?? Color.clear, lineWidth: model.borderWidth ?? 0.0)
            )
        }
        .cornerRadius(model.cornerRadius ?? 0.0)
        .ignoresSafeArea(.all, edges: model.disregardSafeArea ? [.top, .leading, .trailing, .bottom] : [])
        .shadow(
            color: model.shadowColor ?? Color.clear,
            radius: model.shadowRadius ?? 0.0,
            x: model.shadowX ?? 0.0,
            y: model.shadowY ?? 0.0
        )
    }
}

#Preview {
    @State var model = ContainerModel()
    model.update(data: [
        "spacing": 23,
        "alignment": "Center",
        "color": [255, 10, 10, 255],
        "padding_top": 10,
        "padding_left": 40,
        "corner_radius": 10,
        "shadow_radius": 10,
        "shadow_color": [0, 0, 0, 90],
        "border_color": [0, 255, 0, 255],
        "border_width": 4,
        "children": [
            [ "type": "Label", "def": [ "text": "Foo hoo" ] ],
            [ "type": "Label", "def": [ "text": "Bar" ] ],
        ]
    ])
    
    return Container(model: model)
}

#Preview {
    @State var model = ContainerModel()
    model.update(data: [
        "spacing": 23,
        "alignment": "Center",
        "color": [255, 10, 10, 255],
        "padding_top": 100,
        "padding_left": 0,
        "corner_radius": 10,
        "shadow_radius": 10,
        "shadow_color": [0, 0, 0, 90],
        "border_color": [0, 255, 0, 255],
        "border_width": 4,
        "disregard_safe_area": true,
        "children": [
            [ "type": "Label", "def": [ "text": "Foo hoo" ] ],
            [ "type": "Space", "def": [:] ],
            [ "type": "Label", "def": [ "text": "Bar" ] ],
        ]
    ])
    
    return Container(model: model)
}

#Preview {
    let model = ComponentModel()
    model.update(data: [
        "type": "Container",
        "def": [
            "disregard_safe_area": true,
            "children": [
                [ "type": "Container", "def": [ "src": "https://plus.unsplash.com/premium_photo-1669050701946-d34455dce075" ] ]
            ]
        ]
    ])
    
    return Component(model: model)
}
