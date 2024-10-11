import Foundation

class Bridge {
    var onError: ((String) -> Void)?
    var onNoInternet: (() -> Void)?
    var onHasInternet: (() -> Void)?
    var onObjectSet: ((String, Object) -> Void)?
    var onObjectRemoved: ((String) -> Void)?
    var onThemeSet: ((Theme) -> Void)?
    var onDidLoad: (() -> Void)?
    
    private var isOffline = false
    private var url: URL?
    private var websocketTask: URLSessionWebSocketTask?
    
    func start(url: String) {
        let uuid = UUID().uuidString
        print("starting session \(uuid)")
        
        guard let url = URL(string: "\(url)?session_id=\(uuid)") else {
            callError("Invalid url: \(url)")
            return
        }
        
        self.url = url
        self.connect()
    }
    
    func watch(_ id: String) {
        sendMessage(OutgoingMessage<Never>.watch(OutgoingMessage.Watch(id: id)))
    }
    
    func unwatch(_ id: String) {
        sendMessage(OutgoingMessage<Never>.unwatch(OutgoingMessage.Unwatch(id: id)))
    }
    
    func fireEvent<T: Encodable>(key: String, data: T) {
        sendMessage(OutgoingMessage.emitEvent(OutgoingMessage.EmitEvent(key: key, data: data)))
    }
    
    private func callError(_ message: String) {
        if let onError = self.onError {
            onError(message)
        } else {
            print("Unhandled error: \(message)")
        }
    }
    
    private func sendMessage<T: Encodable>(_ message: OutgoingMessage<T>) {
        guard let task = self.websocketTask else {
            print("must call start before watch or fireEvent")
            return
        }
        
        let encoder = JSONEncoder()
        encoder.keyEncodingStrategy = .convertToSnakeCase
        
        guard let json = try? encoder.encode(message) else {
            print("Failed to encode")
            return
        }
        
        task.send(.string(String(decoding: json, as: UTF8.self))) { error in
            if let error = error {
                print("An error occurred: \(error)")
            }
        }
    }
    
    private func recieveMessage() {
        websocketTask?.receive { result in
            switch result {
            case .success(let message):
                if self.isOffline {
                    print("Websocket connected")
                    self.isOffline = false
                    
                    if let onHasInternet = self.onHasInternet {
                        onHasInternet()
                    }
                }
                
                switch message {
                case .string(let data):
                    for message in self.parseIncomingJson(data: data) {
                        self.handleIncomingMessage(message)
                    }
            
                    self.recieveMessage()
                default:
                    self.callError("Non-binary message type recieved")
                }
            case .failure(let error):
                print("Socket error: \(error.localizedDescription)")
                
                if error.localizedDescription == "Could not connect to the server." && self.onNoInternet != nil {
                    print("Websocket connection failed")
                    
                    self.isOffline = true
                    self.onNoInternet!()
                    self.queueRetry()
                } else if error.localizedDescription == "The operation couldn’t be completed. Socket is not connected" {
                    // the socket was suddenly closed (server probably restarted, internet was lost, or ios killed the connection)
                    // if we can reconnect, no error will flash to the user. If not, they will get the standard "no internet" behavior
                    self.queueRetry()
                } else {
                    self.callError(error.localizedDescription)
                }
            }
        }
    }
    
    private func parseIncomingJson(data: String) -> [IncomingMessage] {
        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        
        do {
            return try decoder.decode([IncomingMessage].self, from: Data(data.utf8))
        } catch {
            self.callError("Failed to parse json response")
            
            print("failed to parse json of incoming message: \(error). JSON: \(data)")
            
            return []
        }
    }
    
    private func queueRetry() {
        self.websocketTask?.cancel()
        
        DispatchQueue.main.asyncAfter(deadline: .now() + 3) {
            print("Retrying websocket connnection...")
            self.connect()
        }
    }
    
    private func connect() {
        guard let url = url else {
            print("Must call .start() before .connect()")
            return
        }
        
        websocketTask = URLSession(configuration: .default).webSocketTask(with: url)
        websocketTask?.resume()
        recieveMessage()
    }

    private func handleIncomingMessage(_ message: IncomingMessage) {
        switch message {
        case .initialize(let initalizeMessage):
            guard let onDidLoad = self.onDidLoad else {
                print("Init was sent, but nobody was listening for onDidLoad")
                return
            }
            
            guard let onThemeSet = self.onThemeSet else {
                print("Init theme was set, but nobody was listening: \(initalizeMessage.theme)")
                return
            }
            
            guard let onObjectSet = self.onObjectSet else {
                print("Init objects were set, but nobody was listening: \(initalizeMessage.objects)")
                return
            }
            
            for (id, object) in initalizeMessage.objects {
                onObjectSet(id, object)
            }
            
            onThemeSet(initalizeMessage.theme)
            
            onDidLoad()
        case .removeObject(let removeObjectMessage):
            guard let onObjectRemoved = self.onObjectRemoved else {
                print("An object was removed, but nobody was listening")
                return
            }
            
            onObjectRemoved(removeObjectMessage.id)
        case .setObject(let setObjectMessage):
            guard let onObjectSet = self.onObjectSet else {
                print("An object was set, but nobody was listening")
                return
            }
            
            onObjectSet(setObjectMessage.id, setObjectMessage.object)
        case .setTheme(let setThemeMessage):
            guard let onThemeSet = self.onThemeSet else {
                print("The theme was set, but nobody was listening")
                return
            }
            
            onThemeSet(setThemeMessage.theme)
        case .acknowledge(let acknowledgeMessage):
            print("Acknowledge:", acknowledgeMessage)
        }
    }
}

private enum OutgoingMessage<EventData: Encodable>: Encodable {
    struct Watch: Encodable {
        let id: String
    }
    case watch(Watch)
    
    struct Unwatch: Encodable {
        let id: String
    }
    case unwatch(Unwatch)
    
    struct EmitEvent<T: Encodable>: Encodable {
        let key: String
        let data: T
    }
    case emitEvent(EmitEvent<EventData>)
    
    func encode(to encoder: any Encoder) throws {
        var container = encoder.container(keyedBy: EnumKeys.self)
        
        switch self {
        case .watch(let watch):
            try container.encode("watch", forKey: .kind)
            try container.encode(watch, forKey: .def)
        case .unwatch(let unwatch):
            try container.encode("unwatch", forKey: .kind)
            try container.encode(unwatch, forKey: .def)
        case .emitEvent(let emitEvent):
            try container.encode("emit_event", forKey: .kind)
            try container.encode(emitEvent, forKey: .def)
        }
    }
}

private enum IncomingMessage: Decodable {
    struct Init: Decodable {
        let theme: Theme
        let objects: [String: Object]
    }
    case initialize(Init)
    
    struct RemoveObject: Decodable {
        let id: String
    }
    case removeObject(RemoveObject)
    
    struct SetObject: Decodable {
        let id: String
        let object: Object
    }
    case setObject(SetObject)

    struct SetTheme: Decodable {
        let theme: Theme
    }
    case setTheme(SetTheme)

    struct Acknowledge: Decodable {
        let requestId: UUID?
        let error: String?
        let retryAfterSeconds: Int?
    }
    case acknowledge(Acknowledge)
    
    init(from decoder: any Decoder) throws {
        let container = try decoder.container(keyedBy: EnumKeys.self)
        let kind = try container.decode(String.self, forKey: .kind)
        
        switch kind {
        case "init":
            self = .initialize(try container.decode(Init.self, forKey: .def))
        case "remove_object":
            self = .removeObject(try container.decode(RemoveObject.self, forKey: .def))
        case "set_object":
            self = .setObject(try container.decode(SetObject.self, forKey: .def))
        case "set_theme":
            self = .setTheme(try container.decode(SetTheme.self, forKey: .def))
        case "acknowledge":
            self = .acknowledge(try container.decode(Acknowledge.self, forKey: .def))
        default:
            throw DecodingError.dataCorruptedError(forKey: .kind, in: container, debugDescription: "Unknown kind: \(kind)")
        }
    }
}
