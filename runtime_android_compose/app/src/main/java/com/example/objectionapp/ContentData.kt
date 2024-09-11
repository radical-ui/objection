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
data class Headline(
    @SerialName("text") val text: String
)

@Serializable
data class Quote(
    @SerialName("text") val text: String,
    @SerialName("attribution") val attribution: String,
    @SerialName("surface") val surface: String,
    @SerialName("attribution_surface") val attributionSurface: String,
)

//@Serializable
//enum class ObjectPreviewStyle {
//    @SerialName("search") SEARCH,
//    @SerialName("card") CARD,
//    @SerialName("embed") EMBED,
//}

@Serializable
data class CallToAction(
    @SerialName("title") val title: String,
    @SerialName("icon") val icon: String? = null,
    @SerialName("target_object") val targetObject: String,
    @SerialName("surface") val surface: String,
)

@Serializable
data class ObjectGroup(
    @SerialName("title") val title: String,
    @SerialName("description") val description: String?,
    @SerialName("previews") val previews: List<String>,
)

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class Content {
    @Serializable
    @SerialName("paragraph")
    data class ParagraphContent(val def: Paragraph) : Content()

    @Serializable
    @SerialName("headline")
    data class HeadlineContent(val def: Headline) : Content()

    @Serializable
    @SerialName("quote")
    data class QuoteContent(val def: Quote) : Content()

    @Serializable
    @SerialName("object_preview")
    data class ObjectPreviewContent(val def: ObjectGroup) : Content()

    @Serializable
    @SerialName("call_to_action")
    data class CallToActionContent(val def: CallToAction) : Content()

    @Serializable
    @SerialName("object_group")
    data class ObjectGroupContent(val def: ObjectGroup) : Content()
}
