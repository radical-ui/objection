package com.example.objectionapp

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.json.Json

class ObjectStore<T>(private val bridge: Bridge, private val logger: Logger, private var deserializer: DeserializationStrategy<T>) {
    private val objects = hashMapOf<String, Listener<T?>>()
    private val json = Json { ignoreUnknownKeys = true; isLenient = true }

    init {
        bridge.onObjectSet.listen(ListenId()) { (objectId, obj) ->
            val listener = objects[objectId]

            if (listener != null) {
                listener.emit(json.decodeFromJsonElement(deserializer, obj))
            } else {
                logger.warn("received object that was not watched: $objectId")
            }
        }

        bridge.onObjectRemoved.listen(ListenId()) { objectId ->
            objects[objectId]?.emit(null)
            objects.remove(objectId)
        }
    }

    fun getCurrentObject(id: String): T? {
        return objects[id]?.getLastValue()
    }

    fun listen(listenId: ListenId, objectId: String, callback: (T?) -> Unit) {
        val listener = objects[objectId] ?: createListener(objectId)

        listener.listen(listenId, callback)
    }

    fun removeListener(listenId: ListenId, objectId: String) {
        objects[objectId]?.removeListener(listenId)
    }

    fun preload(key: String, value: T? = null) {
        val listener = objects[key] ?: createListener(key)

        value?.let { listener.emit(it) }
    }

    private fun createListener(objectId: String): Listener<T?> {
        bridge.watch(objectId) { logger.info("Watched $objectId") }

        val listener = Listener<T?>(logger) {
            bridge.unwatch(objectId) { logger.info("Unwatched: $objectId") }
        }

        objects[objectId] = listener

        return listener
    }
}
