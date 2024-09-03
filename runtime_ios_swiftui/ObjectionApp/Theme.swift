import Foundation

struct Theme: Decodable {
    let tabBar: TabBar?
    
    static func empty() -> Theme {
        return Theme(tabBar: nil)
    }
}

struct TabBar: Decodable {
    let objects: [String]
}
