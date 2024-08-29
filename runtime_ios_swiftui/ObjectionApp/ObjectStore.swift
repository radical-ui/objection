import Foundation


class ObjectStore {
    let bridge: Bridge
    
    var onThemeChanged: ((Theme) -> Void)?
    
    private var objectListeners = [UUID: ObjectListener]()
    private var objects = [String: Object]()
    
    init(bridge: Bridge) {
        self.bridge = bridge
        
        bridge.onObjectSet = { id, object in
            self.objects[id] = object
            self.noteObjectUpdate(id: id)
        }
        
        bridge.onObjectRemoved = {id in
            self.objects.removeValue(forKey: id)
            self.noteObjectUpdate(id: id)
        }
        
        bridge.onThemeSet = { theme in
            guard let onThemeChanged = self.onThemeChanged else {
                print("The ObjectStore theme changed, but nobody was listening")
                return
            }
            
            onThemeChanged(theme)
        }
    }
    
    func listen(listen_id: UUID, path: String, callback: @escaping ([Object]) -> Void) {
        let path = ObjectPath(path: path)
        let listener = ObjectListener(path: path, callback: callback)
        
        listener.callback(getMatchingObjects(path: listener.path))
        objectListeners[listen_id] = listener
    }
    
    func removeListener(listen_id: UUID) {
        objectListeners.removeValue(forKey: listen_id)
    }

    private func noteObjectUpdate(id: String) {
        for (_, listener) in objectListeners {
            if listener.path.match(id: id) {
                listener.callback(getMatchingObjects(path: listener.path))
            }
        }
    }

    private func getMatchingObjects(path: ObjectPath) -> [Object] {
        if let exactId = path.getExactId() {
            if let object = objects[exactId] {
                return [object]
            } else {
                return []
            }
        }
        
        var objects = [Object]()
        
        for (id, object) in self.objects {
            if path.match(id: id) {
                objects.append(object)
            }
        }
        
        return objects
    }
}

private struct ObjectListener {
    let path: ObjectPath
    let callback: ([Object]) -> Void
}
