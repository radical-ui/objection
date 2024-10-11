import Foundation

struct ObjectPath: Hashable, Equatable {
    enum Component: Hashable, Equatable {
        case literal(String)
        case dynamic
    }
    
    private let components: [Component]
    private let exactPath: String?
    
    init(path: String) {
        var hasWild = false
        
        self.components = path.components(separatedBy: "/").map { segment in
            if segment == "*" {
                hasWild = true
                return Component.dynamic
            } else {
                return Component.literal(segment)
            }
        }
        
        if !hasWild {
            self.exactPath = path
        } else {
            self.exactPath = nil
        }
    }
    
    func getExactId() -> String? {
        exactPath
    }
    
    func match(id: String) -> Bool {
        let segments = id.components(separatedBy: "/")
        
        if segments.count != components.count {
            return false
        }
        
        for (index, segment) in segments.enumerated() {
            let component = components[index]
            
            if case .literal(let string) = component {
                if string != segment {
                    return false
                }
            }
        }
        
        return true
    }
}
