package com.example.objectionapp

import java.util.UUID

class Session {
    private var sessionId: UUID = UUID.randomUUID()
    private var scope = ""

    fun pushLog(message: String) {
        println("[$sessionId] $message")
    }

    fun getId(): String {
        return sessionId.toString()
    }
}
