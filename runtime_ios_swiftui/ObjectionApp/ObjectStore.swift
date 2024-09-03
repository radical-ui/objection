import Foundation


class ObjectStore {
    let bridge: Bridge
    
    var onThemeChanged: ((Theme) -> Void)?
    
    // The objects that are being listened to, with a reference to the listen id. This should reflect what is currently being "watched" by the server
    private var listenedObjects = [String: [UUID]]()
    
    // The object listeners
    private var objectListeners = [UUID: ObjectListener]()
    
    // All the objects that have been sent down from the server
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
    
    func listen(listenId: UUID, objectIds: [String], callback: @escaping ([Object]) -> Void) {
        let listener = ObjectListener(ids: objectIds, callback: callback)

        listener.callback(getMatchingObjects(ids: objectIds))
        objectListeners[listenId] = listener

        for id in objectIds {
            if var existingListeners = listenedObjects[id] {
                existingListeners.append(listenId)
            } else {
                bridge.watch(id)
                listenedObjects[id] = [listenId]
            }
        }
    }

    func removeListener(listenId: UUID) {
        if let listener = objectListeners[listenId] {
            objectListeners.removeValue(forKey: listenId)
            
            for id in listener.ids {
                if var existingListeners = listenedObjects[id] {
                    if let thisListenerIndex = existingListeners.firstIndex(of: listenId) {
                        existingListeners.remove(at: thisListenerIndex)
                        
                        if existingListeners.isEmpty {
                            bridge.unwatch(id)
                        }
                    } else {
                        print("Removed a listener that existed for '\(id)', but wasnt linked to the listenedObjects. Something is borked")
                    }
                } else {
                    bridge.unwatch(id)
                    print("Removed a listener that existed, but had no entries were in listenedObjects. Unwatched for extra measure, but something is borked")
                }
            }
        }
    }

    private func noteObjectUpdate(id: String) {
        guard let listenerIds = listenedObjects[id] else {
            print("Got an update for '\(id)' before it was needed")
            return
        }

        for listenerId in listenerIds {
            if let listener = objectListeners[listenerId] {
                listener.callback(getMatchingObjects(ids: listener.ids))
            } else {
                print("Listener for object '\(id)' was referenced but did not exist")
            }
        }
    }

    private func getMatchingObjects(ids: [String]) -> [Object] {
        var objects = [Object]()
        
        for id in ids {
            if let object = self.objects[id] {
                objects.append(object)
            }
        }
        
        return objects
    }
}

private struct ObjectListener {
    let ids: [String]
    let callback: ([Object]) -> Void
}
