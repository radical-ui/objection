package com.example.objectionapp

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator

@Serializable
data class Paragraph(
    @SerialName("text") val text: String
)

@Serializable
data class Quote(
    @SerialName("text") val text: String,
    @SerialName("author") val author: String
)

@Serializable
data class ObjectPreview(
    @SerialName("object_id") val objectId: String
)

@Serializable
data class CallToAction(
    @SerialName("title") val title: String,
    @SerialName("icon") val icon: String? = null,
    @SerialName("target_object") val targetObject: String
)

@Serializable
data class ObjectGroup(
    @SerialName("paragraph") val title: String,
    @SerialName("description") val description: Boolean,
    @SerialName("object_scope") val objectScope: String
)

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class Content {
    @Serializable
    @SerialName("paragraph")
    data class ParagraphContent(val def: Paragraph) : Content()

    @Serializable
    @SerialName("quote")
    data class QuoteContent(val def: Quote) : Content()

    @Serializable
    @SerialName("object_preview")
    data class ObjectPreviewContent(val def: ObjectPreview) : Content()

    @Serializable
    @SerialName("call_to_action")
    data class CallToActionContent(val def: CallToAction) : Content()

    @Serializable
    @SerialName("object_group")
    data class ObjectGroupContent(val def: ObjectGroup) : Content()
}

@Serializable
data class Object(
    @SerialName("title") val title: String? = null,
    @SerialName("subtitle") val subtitle: String? = null,
    @SerialName("description") val description: String? = null,
    @SerialName("icon") val icon: String? = null,
    @SerialName("content") val content: List<Content>,
    @SerialName("actions") val actions: List<Action>
)

@Serializable
enum class ActionKind {
    @SerialName("danger")
    DANGER,

    @SerialName("success")
    SUCCESS,

    @SerialName("normal")
    NORMAL
}

@Serializable
data class Action(
    @SerialName("id") val id: String,
    @SerialName("kind") val kind: ActionKind,
    @SerialName("title") val title: String,
    @SerialName("icon") val icon: String? = null
)
