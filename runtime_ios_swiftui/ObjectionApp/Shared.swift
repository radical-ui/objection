import SwiftUI

func buildColor(_ value: Any) -> Color? {
    guard let array = value as? [Int] else {
        print("color value is not an array")
        return nil
    }
    
    if array.count != 4 {
        print("color array must have exactly 4 items")
        return nil
    }
    
    return Color(
        red: Double(array[0]) / 255,
        green: Double(array[1]) / 255,
        blue: Double(array[2]) / 255,
        opacity: Double(array[3]) / 255
    )
}

func buildDouble(_ value: Any) -> Double? {
    if let num = value as? Double {
        return num
    }
    
    if let num = value as? Int {
        return Double(num)
    }
    
    print("expected an int or double")
    
    return nil
}

//extension View {
//    func border(edge: Edge, width: CGFloat, color: Color) -> some View {
//        overlay(EdgeBorder(width: width, edge: edge).foregroundColor(color))
//    }
//}

struct EdgeBorder: Shape {
    var width: CGFloat
    var edge: Edge
    
    func path(in rect: CGRect) -> Path {
        switch edge {
        case .top: return Path(.init(x: rect.minX, y: rect.minY, width: rect.width, height: width))
        case .bottom: return Path(.init(x: rect.minX, y: rect.maxY - width, width: rect.width, height: width))
        case .leading: return Path(.init(x: rect.minX, y: rect.minY, width: width, height: rect.height))
        case .trailing: return Path(.init(x: rect.maxX - width, y: rect.minY, width: width, height: rect.height))
        }
    }
}
