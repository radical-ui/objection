package com.example.objectionapp

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator

@Serializable
@ExperimentalSerializationApi
data class Object(
    @SerialName("title") val title: String?,
    @SerialName("engaged_title") val engagedTitle: String?,
    @SerialName("subtitle") val subtitle: String?,
    @SerialName("icon") val icon: String?,
    @SerialName("image") val image: String?,
    @SerialName("content") val content: List<Content>,
    @SerialName("actions") val actions: List<Action>,
    @SerialName("search_object") val searchObject: String?
)

@ExperimentalSerializationApi
@Serializable
@JsonClassDiscriminator("kind")
sealed class ActionKind {
    @SerialName("danger")
    data object Danger: ActionKind()

    @SerialName("success")
    data object Success: ActionKind()

    @SerialName("normal")
    data object Normal: ActionKind()
}

@Serializable
@ExperimentalSerializationApi
data class Action(
    @SerialName("kind") val kind: ActionKind,
    @SerialName("title") val title: String,
    @SerialName("icon") val icon: String?,
    @SerialName("process") val process: ActionProcess,
)

@Serializable
data class PerformOperationActionProcess(
    @SerialName("key") val key: String
)

@Serializable
data class ShowObjectActionProcess(
    @SerialName("id") val id: String
)

@ExperimentalSerializationApi
@Serializable
@JsonClassDiscriminator("kind")
sealed class ActionProcess {
    @Serializable
    @SerialName("perform_operation")
    data class PerformOperation(val def: PerformOperationActionProcess) : ActionProcess()

    @Serializable
    @SerialName("show_object")
    data class ShowObject(val def: ShowObjectActionProcess) : ActionProcess()
}
