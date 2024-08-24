import Foundation

class Bridge {
    static var shared = Bridge()
    
    var onInitial: ((_: Any) -> Void)?
    var onError: ((_: String) -> Void)?
    var onNoInternet: (() -> Void)?
    var onHasInternet: (() -> Void)?
    
    private var isOffline = false
    private var url: URL?
    private var websocketTask: URLSessionWebSocketTask?
    private var listeners: [String: (_ data: [String: Any]) -> Void] = [:]
    
    func onUpdate(_ id: String, handler: @escaping (_ data: [String: Any]) -> Void) {
        self.listeners[id] = handler
    }
    
    func removeListener(_ id: String) {
        self.listeners.removeValue(forKey: id)
    }
    
    private func callError(_ message: String) {
        if let onError = self.onError {
            onError(message)
        } else {
            print("Unhandled error: \(message)")
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
                case .data(let data):
                    guard let json = try? JSONSerialization.jsonObject(with: data) else {
                        self.callError("Failed to parse json response")
                        return
                    }

                    guard let object = json as? [String: Any] else {
                        self.callError("Json message is not an object")
                        return
                    }

                    self.handleIncomingMessage(message: object)
                    self.recieveMessage()
                default:
                    self.callError("Non-binary message type recieved")
                }
            case .failure(let error):
                if error.localizedDescription == "Could not connect to the server." && self.onNoInternet != nil {
                    print("Websocket connection failed")
                    
                    self.isOffline = true
                    self.onNoInternet!()
                    self.queueRetry()
                } else {
                    self.callError(error.localizedDescription)
                }
            }
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
    
    func start(url: String) {
        guard let url = URL(string: url) else {
            callError("Invalid url: \(url)")
            return
        }
        
        self.url = url
//        self.connect()
    }

    func handleIncomingMessage(message: [String: Any]) {
        guard let operation = message["operation"] as? String else {
            callError("Incoming message did not contain a valid operation")
            return
        }
        
        if operation == "Initial" {
            guard let state = message["state"] else {
                callError("Incoming 'Initial' message did not contain the state")
                return
            }
            
            if let onInitial = self.onInitial {
                onInitial(state)
            } else {
                print("Recieved initial state, but nobbody was listening for it")
            }
        } else if operation == "Update" {
            guard let id = message["id"] as? String else {
                callError("Incomming 'Update' message did not contain an id")
                return
            }
            
            guard let listener = self.listeners[id] else {
                callError("No listeners were listening for \(id)")
                return
            }
            
            guard let state = message["state"] as? [String: Any] else {
                callError("Incoming 'Update' message did not contain the state")
                return
            }
            
            listener(state)
        }
    }
}
