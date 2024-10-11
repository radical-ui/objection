import Foundation

struct Paragraph: Decodable, Hashable, Equatable {
    let text: String
}

struct Quote: Decodable, Hashable, Equatable {
    let text: String
    let author: String
}

struct ObjectPreview: Decodable, Hashable, Equatable {
    let objectId: String
}

struct CallToAction: Decodable, Hashable, Equatable {
    let title: String
    let icon: String?
    let targetObject: String
}

struct ObjectGroup: Decodable, Hashable, Equatable {
    let title: String
    let description: Bool
    let objectScope: String
}

enum Content: Decodable, Hashable, Equatable {
    case paragraph(Paragraph)
    case quote(Quote)
    case objectPreview(ObjectPreview)
    case callToAction(CallToAction)
    case objectGroup(ObjectGroup)
    
    init(from decoder: any Decoder) throws {
        let container = try decoder.container(keyedBy: EnumKeys.self)
        let kind = try container.decode(String.self, forKey: .kind)
        
        switch kind {
        case "paragraph":
            self = .paragraph(try container.decode(Paragraph.self, forKey: .def))
        case "quote":
            self = .quote(try container.decode(Quote.self, forKey: .def))
        case "object_preview":
            self = .objectPreview(try container.decode(ObjectPreview.self, forKey: .def))
        case "call_to_action":
            self = .callToAction(try container.decode(CallToAction.self, forKey: .def))
        case "object_group":
            self = .objectGroup(try container.decode(ObjectGroup.self, forKey: .def))
        default:
            throw DecodingError.dataCorruptedError(forKey: .kind, in: container, debugDescription: "Unknown kind")
        }
    }
}

struct Object: Decodable, Hashable, Equatable {
    let title: String?
    let subtitle: String?
    let description: String?
    let icon: String?
    let content: [Content]
    let actions: [Action]
}

enum ActionKind: String, Hashable, Decodable {
    case danger = "danger"
    case success = "success"
    case normal = "normal"
}

struct Action: Decodable, Hashable, Equatable, Identifiable {
    let id: String
    let kind: ActionKind
    let title: String
    let icon: String?
}
