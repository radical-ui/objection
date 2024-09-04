package com.example.objectionapp

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class Theme(
    @SerialName("tab_bar") val tabBar: TabBar?,
    @SerialName("corner_rounding") val cornerRounding: CornerRounding
) {
    fun getInitialObjectId(): String? {
        return tabBar?.objects?.first()
    }

    companion object {
        fun testDefault(): Theme {
            return Theme(tabBar = null, cornerRounding = CornerRounding.ROUND)
        }
    }
}

@Serializable
enum class CornerRounding {
    @SerialName("sharp")
    SHARP,

    @SerialName("round")
    ROUND,

    @SerialName("extra_round")
    EXTRA_ROUND,
}

@Serializable
data class TabBar(
    @SerialName("objects") val objects: List<String>
)
