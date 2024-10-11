package com.example.objectionapp

class Logger(private var session: Session, private var scope: String) {
    fun critical(message: String) {
        session.pushLog("CRITICAL: $message")
    }

    fun error(message: String) {
        session.pushLog("ERROR: $message")
    }

    fun warn(message: String) {
        session.pushLog("WARN: $message")
    }

    fun info(message: String) {
        session.pushLog("INFO: $message")
    }

    fun scope(name: String): Logger {
        return Logger(session = session, scope = "$scope.$name")
    }
}
