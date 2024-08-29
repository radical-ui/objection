import Foundation

struct Theme: Decodable {
    let tabBar: TabBar?
}

struct TabBar: Decodable {
    let items: [TabBarItem]
}

struct TabBarItem: Decodable {
    let icon: String
    let label: String
    let objectId: String
}
