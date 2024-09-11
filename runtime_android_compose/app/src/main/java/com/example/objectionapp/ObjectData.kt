package com.example.objectionapp

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator

@Serializable
data class Object(
    @SerialName("title") val title: String?,
    @SerialName("media") val media: List<MediaItem>,
    @SerialName("functions") val functions: List<Action>,
    @SerialName("pinned_preview") val pinnedPreview: PinnedObjectPreview?,

//    I think that it may not be a good idea to have an engaged title
//    @SerialName("engaged_title") val engagedTitle: String?,
)

@Serializable
data class MediaItem (
    @SerialName("name") val name: String,
    @SerialName("source") val source: MediaSource,
    @SerialName("is_editable") val isEditable: Boolean
)

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class MediaSource {
    @Serializable
    data class ImageDef(
        @SerialName("url") val url: String
    )

    @Serializable
    data class VideoDef(
        @SerialName("url") val url: String
    )

    @Serializable
    data class BackgroundDef(
        @SerialName("background") val background: BackgroundData
    )

    @Serializable
    data class IconDef(
        @SerialName("name") val name: String
    )

    @Serializable
    @SerialName("image")
    data class Image(val def: ImageDef) : MediaSource()

    @Serializable
    @SerialName("video")
    data class Video(val def: VideoDef) : MediaSource()

    @Serializable
    @SerialName("video")
    data class Background(val def: BackgroundData) : MediaSource()

    @Serializable
    @SerialName("icon")
    data class Icon(val def: IconDef) : MediaSource()
}

@Serializable
data class PinnedObjectPreview(
    @SerialName("object_id") val objectId: String,
    @SerialName("options") val options: ObjectPreviewOptions
)

@Serializable
data class ObjectPreviewOptions(
    @SerialName("surface") val surface: String?,
    @SerialName("style") val style: ObjectPreviewStyle,
)

@OptIn(ExperimentalSerializationApi::class)
@Serializable
@JsonClassDiscriminator("kind")
sealed class ObjectPreviewStyle {
    data class CardDef(
        @SerialName("title") val title: String
    )

    data class Card(val def: CardDef)
}

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

@OptIn(ExperimentalSerializationApi::class)
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
