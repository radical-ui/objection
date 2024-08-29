//import SwiftUI
//
//class ImageModel: ObservableObject {
//    @Published var url: URL?
//    @Published var width: Double?
//    @Published var height: Double?
//    @Published var fit: Bool?
//    
//	func update(data: [String: Any]) {
//        if let src = data["src"] {
//            self.url = buildUrl(src)
//        }
//        
//        if let width = data["width"] {
//            self.width = buildDouble(width)
//        }
//        
//        if let height = data["height"] {
//            self.height = buildDouble(height)
//        }
//        
//        if let fit = data["fit"] as? Bool {
//            self.fit = fit
//        }
//	}
//}
//
//struct Image: View {
//    @ObservedObject var model: ImageModel
//    
//	var body: some View {
//        if let url = model.url {
//            SwiftUI.AsyncImage(url: url) { phase in
//                if let image = phase.image {
//                    var raw = image.resizable().aspectRatio(contentMode: self.model.fit == true ? .fit : .fill)
//                    
//                    if self.model.width != nil && self.model.height != nil {
//                        raw.frame(width: self.model.width!, height: self.model.height!).clipped()
//                    } else if let width = self.model.width {
//                        raw.frame(width: width).clipped()
//                    } else if let height = self.model.height {
//                        raw.frame(height: height).clipped()
//                    } else {
//                        raw
//                    }
//                }
//            }
//        }
//	}
//}
//
//#Preview {
//    let model = ImageModel()
//    model.update(data: [
//        "width": 100,
//        "height": 100,
//        "fit": true,
//        "src": "https://raw.githubusercontent.com/sveltejs/branding/master/svelte-logo.svg"
//    ])
//    
//    return Image(model: model)
//}
//
//#Preview {
//    let model = ComponentModel()
//    model.update(data: [
//        "type": "Container",
//        "def": [
//            "disregard_safe_area": true,
//            "children": [
//                [ "type": "Image", "def": [ "src": "https://plus.unsplash.com/premium_photo-1669050701946-d34455dce075" ] ]
//            ]
//        ]
//    ])
//    
//    return Component(model: model)
//}
