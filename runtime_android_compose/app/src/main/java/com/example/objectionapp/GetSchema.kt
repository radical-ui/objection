package com.example.objectionapp

import android.os.Build
import androidx.annotation.RequiresApi
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.Json
import java.nio.file.Paths
import kotlin.reflect.typeOf

@OptIn(ExperimentalSerializationApi::class)
@RequiresApi(Build.VERSION_CODES.O)
fun main() {
	val json = Json { ignoreUnknownKeys = true; isLenient = true; encodeDefaults = true; prettyPrint = true; prettyPrintIndent = "\t" }
	val schema = getSchema(typeOf<Object>())
	val text = json.encodeToString(Schema.serializer(), schema)
	val path = Paths.get("schema.json")

	path.toFile().writeText(text)

	println("Wrote schema to ${path.toAbsolutePath()}")
}