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

func buildUrl(_ value: Any) -> URL? {
    guard let string = value as? String else {
        print("expected a string")
        return nil
    }
    
    guard let url = URL(string: string) else {
        print("expected a valid url")
        return nil
    }
    
    return url
}

enum EnumKeys: String, CodingKey {
    case kind, def
}
