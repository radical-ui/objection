package com.example.objectionapp

import java.util.UUID

class Listener<T>(private var logger: Logger) {
    private var listeners = mutableMapOf<ListenId, (T) -> Unit>()

    fun listen(id: ListenId, callback: (T) -> Unit) {
        listeners[id] = callback
    }

    fun removeListener(id: ListenId) {
        listeners.remove(id)
    }

    fun emit(data: T) {
        if (listeners.isEmpty()) {
            logger.warn("Emitted '$data' to listeners, but nobody was listening")
        }

        for (entry in listeners) {
            entry.value.invoke(data)
        }
    }
}

class ListenId {
    private var uuid: UUID = UUID.randomUUID()
}
