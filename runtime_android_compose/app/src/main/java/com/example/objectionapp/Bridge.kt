package com.example.objectionapp

import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.Job
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonClassDiscriminator
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonNull
import okhttp3.OkHttpClient
import okhttp3.WebSocket
import okhttp3.Request
import okhttp3.Response
import okhttp3.WebSocketListener
import java.io.EOFException
import java.net.ConnectException
import java.net.SocketTimeoutException
import java.util.UUID
import kotlin.coroutines.CoroutineContext
import kotlinx.serialization.Serializable

class Bridge(private var logger: Logger, private var session: Session) : CoroutineScope {
	private var job: Job = Job()
	override val coroutineContext: CoroutineContext
		get() = Dispatchers.Main + job

	var onError = Listener<String>(logger = logger)
	var onHasInternet = Listener<Boolean>(logger = logger)
	var onObjectSet = Listener<Pair<String, JsonElement>>(logger = logger)
	var onObjectRemoved = Listener<String>(logger = logger)

	private var isOffline = false
	private var url: String? = null
	private var websocket: WebSocket? = null
	private var isRunning = false
	private val client = OkHttpClient()
	private val json = Json { ignoreUnknownKeys = true; isLenient = true; encodeDefaults = true }

	fun start(url: String) {
		if (!isRunning) {
			logger.info("starting")

			this.websocket
			val fullUrl = "$url?session_id=${session.getId()}"
			this.url = fullUrl
			connect()
		} else {
			logger.info("start called again, but skipping because bridge is already running")
		}
	}

	fun watch(objectId: String, onComplete: () -> Unit) {
		sendMessage(
			OutgoingMessage.Watch(
				requestId = listenForAcknowledgement(onComplete),
				id = objectId,
			)
		)
	}

	fun unwatch(objectId: String, onComplete: () -> Unit) {
		sendMessage(
			OutgoingMessage.Unwatch(
				requestId = listenForAcknowledgement(onComplete),
				id = objectId,
			)
		)
	}

	fun emitNullEvent(objectId: String, key: String, onComplete: () -> Unit) {
		sendMessage(
			OutgoingMessage.EmitEvent(
				requestId = listenForAcknowledgement(onComplete),
				objectId = objectId,
				key = key,
				data = JsonNull
			)
		)
	}

	fun emitEvent(
		objectId: String,
		key: String,
		data: JsonElement,
		onComplete: () -> Unit
	) {
		sendMessage(
			OutgoingMessage.EmitEvent(
				requestId = listenForAcknowledgement(onComplete),
				objectId = objectId,
				key = key,
				data = data
			)
		)
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
		println("before cancel")
		websocket?.cancel()
		println("after cancel")

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
				isRunning = true

				if (isOffline) {
					logger.info("websocket connected")
					isOffline = false
					onHasInternet.emit(true)
				}
			}

			override fun onMessage(webSocket: WebSocket, text: String) {
				parseIncomingJson(text).forEach { handleIncomingMessage(it) }
			}

			override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
				when (t) {
					is ConnectException, is SocketTimeoutException -> {
						isOffline = true
						onHasInternet.emit(false)
						queueRetry()
					}

					is EOFException -> {
						logger.warn("connection was suddenly dropped")
						queueRetry()
					}

					else -> {
						logger.warn("a socket failed: $t")
						callError(t.localizedMessage ?: "Unknown error")
					}
				}
			}
		})
	}

	private fun handleIncomingMessage(message: IncomingMessage) {
		when (message) {
			is IncomingMessage.RemoveObject -> onObjectRemoved.emit(message.id)
			is IncomingMessage.SetObject -> onObjectSet.emit(Pair(message.id, message.data))
			is IncomingMessage.Acknowledge -> logger.warn("TODO acknowledge: ${message.requestId}")
		}
	}
}

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("$")
sealed class OutgoingMessage {
	@Serializable
	@SerialName("watch")
	data class Watch(
		@SerialName("request_id") val requestId: String,
		val id: String
	) : OutgoingMessage()

	@Serializable
	@SerialName("unwatch")
	data class Unwatch(
		@SerialName("request_id") val requestId: String,
		val id: String
	) : OutgoingMessage()

	@Serializable
	@SerialName("emit_event")
	data class EmitEvent(
		@SerialName("request_id") val requestId: String,
		@SerialName("object_id") val objectId: String,
		val key: String,
		val data: JsonElement
	) : OutgoingMessage()
}

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("$")
sealed class IncomingMessage {
	@Serializable
	@SerialName("remove_object")
	data class RemoveObject(val id: String) : IncomingMessage()

	@Serializable
	@SerialName("set_object")
	data class SetObject(val id: String, val data: JsonElement) : IncomingMessage()

	@Serializable
	@SerialName("acknowledge")
	data class Acknowledge(
		@SerialName("request_id") val requestId: String? = null,
		val error: String? = null,
		@SerialName("retry_after_seconds") val retryAfterSeconds: Int? = null
	) : IncomingMessage()
}
