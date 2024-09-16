package com.example.objectionapp

class ObjectStore(private val bridge: Bridge, private val logger: Logger) {
    // The objects that are being listened to, with a reference to the listen id. This should reflect what is currently being "watched" by the server
    private val listenedObjects = mutableMapOf<String, MutableList<ListenId>>()

    // The object listeners
    private val objectListeners = mutableMapOf<ListenId, ObjectListener>()

    // All the objects that have been sent down from the server
    private val objects = mutableMapOf<String, Object>()

    init {
        bridge.onObjectSet.listen(ListenId()) { (id, obj) ->
            objects[id] = obj
            noteObjectUpdate(id)
        }

        bridge.onObjectRemoved.listen(ListenId()) { id ->
            objects.remove(id)
            noteObjectUpdate(id)
        }
    }

    fun listen(listenId: ListenId, objectIds: List<String>, callback: (List<Pair<String, Object>>) -> Unit) {
        val listener = ObjectListener(ids = objectIds, callback = callback)

        objectListeners[listenId] = listener
        for (id in objectIds) {
            val existingListeners = listenedObjects[id]
            if (existingListeners != null) {
                existingListeners.add(listenId)
            } else {
                bridge.watch(id) {}
                listenedObjects[id] = mutableListOf(listenId)
            }
        }
    }

    fun removeListener(listenId: ListenId) {
        val maybeListener = objectListeners[listenId]

        maybeListener?.let { listener ->
            objectListeners.remove(listenId)

            for (id in listener.ids) {
                val existingListeners = listenedObjects[id]
                if (existingListeners != null) {
                    val thisListenerIndex = existingListeners.indexOf(listenId)
                    if (thisListenerIndex != -1) {
                        existingListeners.removeAt(thisListenerIndex)

                        if (existingListeners.isEmpty()) {
                            bridge.unwatch(id) {}
                        }
                    } else {
                        logger.error("removed a listener that existed for '$id', but wasn't linked to the listenedObjects")
                    }
                } else {
                    bridge.unwatch(id) {}
                    logger.error("removed a listener that existed, but had no entries in listenedObjects. Unwatched for extra measure, but something is broken")
                }
            }
        }
    }

    fun getCurrentObjects(ids: List<String>): List<Pair<String, Object>> {
        val matchingObjects = mutableListOf<Pair<String, Object>>()
        for (id in ids) {
            val obj = objects[id]
            if (obj != null) {
                matchingObjects.add(Pair(id, obj))
            }
        }
        return matchingObjects
    }

    private fun noteObjectUpdate(id: String) {
        val listenerIds = listenedObjects[id]
        if (listenerIds != null) {
            for (listenerId in listenerIds) {
                val listener = objectListeners[listenerId]
                if (listener != null) {
                    listener.callback(getCurrentObjects(listener.ids))
                } else {
                    logger.error("listener for object '$id' was referenced but did not exist")
                }
            }
        } else {
            logger.info("got an update for '$id' before it was needed")
        }
    }
}

private data class ObjectListener(
    val ids: List<String>,
    val callback: (List<Pair<String, Object>>) -> Unit
)
