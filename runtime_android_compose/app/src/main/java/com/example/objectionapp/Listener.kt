package com.example.objectionapp

import java.util.UUID

class Listener<T>(private var logger: Logger, onZeroed: () -> Unit = {}) {
    private var lastValue: T? = null
    private var listeners = mutableMapOf<ListenId, (T) -> Unit>()

    fun getLastValue(): T? {
        return lastValue
    }

    fun listen(id: ListenId, callback: (T) -> Unit) {
        listeners[id] = callback
    }

    fun removeListener(id: ListenId) {
        listeners.remove(id)
    }

    fun emit(data: T) {
        lastValue = data

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
