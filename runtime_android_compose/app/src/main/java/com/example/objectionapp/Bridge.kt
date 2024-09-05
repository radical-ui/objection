package com.example.objectionapp

import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.*
import kotlinx.serialization.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonClassDiscriminator
import okhttp3.OkHttpClient
import okhttp3.WebSocket
import okhttp3.Request
import okhttp3.Response
import okhttp3.WebSocketListener
import java.util.UUID
import kotlin.coroutines.CoroutineContext

class Bridge(private var logger: Logger, private var session: Session) : CoroutineScope {
    private var job: Job = Job()
    override val coroutineContext: CoroutineContext
        get() = Dispatchers.Main + job

    var onError = Listener<String>(logger = logger)
    var onNoInternet = Listener<Unit>(logger = logger)
    var onHasInternet = Listener<Unit>(logger = logger)
    var onObjectSet = Listener<Pair<String, Object>>(logger = logger)
    var onObjectRemoved = Listener<String>(logger = logger)
    var onThemeSet = Listener<Theme>(logger = logger)
    var onDidLoad = Listener<Unit>(logger = logger)

    private var currentTheme = Theme.testDefault()
    private var isOffline = false
    private var url: String? = null
    private var websocket: WebSocket? = null
    private val client = OkHttpClient()
    private val json = Json { ignoreUnknownKeys = true; isLenient = true; encodeDefaults = true }

    fun start(url: String) {
        logger.info("starting")

        val fullUrl = "$url?session_id=${session.getId()}"
        this.url = fullUrl
        connect()
    }

    fun watch(objectId: String, onComplete: () -> Unit) {
        sendMessage(OutgoingMessage.Watch(OutgoingWatchMessage(
            requestId = listenForAcknowledgement(onComplete),
            id = objectId,
        )))
    }

    fun unwatch(objectId: String, onComplete: () -> Unit) {
        sendMessage(OutgoingMessage.Unwatch(OutgoingUnwatchMessage(
            requestId = listenForAcknowledgement(onComplete),
            id = objectId,
        )))
    }

    fun performOperation(objectId: String, key: String, onComplete: () -> Unit) {
        sendMessage(OutgoingMessage.PerformOperation(OutgoingPerformOperationMessage(
            requestId = listenForAcknowledgement(onComplete),
            objectId = objectId,
            key,
        )))
    }

    fun getCurrentTheme(): Theme {
        return currentTheme
    }

    private fun listenForAcknowledgement(callback: () -> Unit): String {
        return UUID.randomUUID().toString()
    }

    private fun callError(message: String) {
        onError.emit(message)
    }

    private fun sendMessage(message: OutgoingMessage) {
        websocket?.let { ws ->
            val jsonMessage = json.encodeToString(OutgoingMessage.serializer(), message)
            ws.send(jsonMessage)
        } ?: logger.error("must call start() before watch() or fireEvent()")
    }

    private fun parseIncomingJson(data: String): List<IncomingMessage> {
        return try {
            json.decodeFromString(data)
        } catch (e: Exception) {
            callError("Failed to parse information from server.")
            logger.critical("failed to parse json of incoming message: ${e.message}. JSON: $data")

            emptyList()
        }
    }

    private fun queueRetry() {
        websocket?.cancel()

        Executors.newSingleThreadScheduledExecutor().schedule({
            logger.info("retrying websocket connection")
            connect()
        }, 3, TimeUnit.SECONDS)
    }

    private fun connect() {
        logger.info("connecting")

        val url = url ?: run {
            logger.error("must call .start() before .connect()")
            return
        }

        val request = Request.Builder().url(url).build()
        websocket = client.newWebSocket(request, object : WebSocketListener() {
            override fun onOpen(webSocket: WebSocket, response: Response) {
                if (isOffline) {
                    logger.info("websocket connected")
                    isOffline = false
                    onHasInternet.emit(Unit)
                }
            }

            override fun onMessage(webSocket: WebSocket, text: String) {
                parseIncomingJson(text).forEach { handleIncomingMessage(it) }
            }

            override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                logger.warn("socket failure: ${t.localizedMessage}")

//                Find out what constitutes a failure due to no internet
//                isOffline = true
//                onNoInternet.emit(Unit)

                callError(t.localizedMessage ?: "Unknown error")
            }
        })
    }

    private fun handleIncomingMessage(message: IncomingMessage) {
        when (message) {
            is IncomingMessage.Initialize -> {
                onDidLoad.emit(Unit)
                onThemeSet.emit(message.def.theme)
                currentTheme = message.def.theme

                message.def.objects.forEach { (id, obj) -> onObjectSet.emit(Pair(id, obj)) }
            }

            is IncomingMessage.RemoveObject -> {
                onObjectRemoved.emit(message.def.id)
            }

            is IncomingMessage.SetObject -> {
                onObjectSet.emit(Pair(message.def.id, message.def.obj))
            }

            is IncomingMessage.SetTheme -> {
                currentTheme = message.def.theme
                onThemeSet.emit(message.def.theme)
            }

            is IncomingMessage.Acknowledge -> {
                logger.warn("TODO acknowledge: ${message.def.requestId}")
            }
        }
    }
}

@Serializable
data class OutgoingWatchMessage(
    @SerialName("request_id") val requestId: String,
    @SerialName("id") val id: String
)

@Serializable
data class OutgoingUnwatchMessage(
    @SerialName("request_id") val requestId: String,
    @SerialName("id") val id: String
)

@Serializable
data class OutgoingPerformOperationMessage(
    @SerialName("request_id") val requestId: String,
    @SerialName("object_id") val objectId: String,
    @SerialName("key") val key: String
)

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class OutgoingMessage {
    @Serializable
    @SerialName("watch")
    data class Watch(val def: OutgoingWatchMessage) : OutgoingMessage()

    @Serializable
    @SerialName("unwatch")
    data class Unwatch(val def: OutgoingUnwatchMessage) : OutgoingMessage()

    @Serializable
    @SerialName("emit_event")
    data class PerformOperation(val def: OutgoingPerformOperationMessage) : OutgoingMessage()
}

@Serializable
data class IncomingInitializeMessage(
    @SerialName("theme") val theme: Theme,
    @SerialName("objects") val objects: Map<String, Object>
)

@Serializable
data class IncomingRemoveObjectMessage(
    @SerialName("id") val id: String,
)

@Serializable
data class IncomingSetObjectMessage(
    @SerialName("id") val id: String,
    @SerialName("object") val obj: Object,
)

@Serializable
data class IncomingSetThemeMessage(
    @SerialName("theme") val theme: Theme
)

@Serializable
data class IncomingAcknowledgeMessage(
    @SerialName("request_id") val requestId: String? = null,
    @SerialName("error") val error: String? = null,
    @SerialName("retry_after_seconds") val retryAfterSeconds: Int? = null
)

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class IncomingMessage {
    @Serializable
    @SerialName("init")
    data class Initialize(val def: IncomingInitializeMessage) : IncomingMessage()

    @Serializable
    @SerialName("remove_object")
    data class RemoveObject(val def: IncomingRemoveObjectMessage) : IncomingMessage()

    @Serializable
    @SerialName("set_object")
    data class SetObject(val def: IncomingSetObjectMessage) : IncomingMessage()

    @Serializable
    @SerialName("set_theme")
    data class SetTheme(val def: IncomingSetThemeMessage) : IncomingMessage()

    @Serializable
    @SerialName("acknowledge")
    data class Acknowledge(val def: IncomingAcknowledgeMessage) : IncomingMessage()
}
