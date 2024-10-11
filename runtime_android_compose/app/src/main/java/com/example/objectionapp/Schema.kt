package com.example.objectionapp

import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialInfo
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PolymorphicKind
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.SerialKind
import kotlinx.serialization.descriptors.StructureKind
import kotlinx.serialization.descriptors.elementDescriptors
import kotlinx.serialization.descriptors.serialDescriptor
import kotlinx.serialization.json.JsonClassDiscriminator
import kotlin.reflect.KClass
import kotlin.reflect.full.createType

@OptIn(ExperimentalSerializationApi::class)
@SerialInfo
@Target(AnnotationTarget.PROPERTY)
annotation class Description(val content: String)

@OptIn(ExperimentalSerializationApi::class)
@SerialInfo
@Target(AnnotationTarget.PROPERTY)
annotation class ObjectReference(val expectedTopLevelVariant: KClass<*>)

@OptIn(ExperimentalSerializationApi::class)
@SerialInfo
@Target(AnnotationTarget.PROPERTY)
annotation class AnyObjectReference

@OptIn(ExperimentalSerializationApi::class)
@SerialInfo
@Target(AnnotationTarget.CLASS)
annotation class IsColor

@OptIn(ExperimentalSerializationApi::class)
@SerialInfo
@Target(AnnotationTarget.CLASS)
annotation class ContentKey(val key: String)

fun getSchema(klass: KClass<*>): Schema {
	return Schema(getItemSchema(serialDescriptor(klass.createType())))
}

@OptIn(ExperimentalSerializationApi::class)
private fun getItemSchema(
	descriptor: SerialDescriptor, annotations: List<Annotation> = listOf()
): ItemSchema {
	return when (descriptor.kind) {
		StructureKind.CLASS -> getClassSchema(descriptor)
		PrimitiveKind.STRING -> getStringSchema(annotations)
		PrimitiveKind.DOUBLE -> ItemSchema.NumberSchema
		PrimitiveKind.FLOAT -> ItemSchema.NumberSchema
		PrimitiveKind.INT -> ItemSchema.NumberSchema
		PrimitiveKind.BOOLEAN -> ItemSchema.BooleanSchema
		StructureKind.LIST -> getListSchema(descriptor)
		PolymorphicKind.SEALED -> getSealedSchema(descriptor)
		SerialKind.ENUM -> throw Exception("Use a sealed class with objects instead of an enum. Failed at: $descriptor")
		else -> throw Exception("unknown item at $descriptor")
	}
}

@OptIn(ExperimentalSerializationApi::class)
private fun getStringSchema(annotations: List<Annotation>): ItemSchema {
	for (annotation in annotations) {
		when (annotation) {
			is ObjectReference -> return ItemSchema.ReferenceSchema(
				expectedTopLevelVariant = serialDescriptor(
					annotation.expectedTopLevelVariant.createType()
				).serialName
			)

			is AnyObjectReference -> return ItemSchema.ReferenceSchema(expectedTopLevelVariant = null)
		}
	}

	return ItemSchema.StringSchema
}

@OptIn(ExperimentalSerializationApi::class)
private fun getSealedSchema(descriptor: SerialDescriptor): ItemSchema.EnumSchema {
	val variants = mutableListOf<EnumVariantSchema>()
	val child = descriptor.getElementDescriptor(1)
	var discriminatorKey: String? = null
	var contentKey: String? = null

	for (variant in child.elementDescriptors) {
		variants.add(
			EnumVariantSchema(
				name = variant.serialName, description = getDescription(listOf()), // FIXME
				type = getItemSchema(variant)
			)
		)
	}

	for (annotation in descriptor.annotations) {
		if (annotation is JsonClassDiscriminator) {
			discriminatorKey = annotation.discriminator
		}
		if (annotation is ContentKey) {
			contentKey = annotation.key
		}
	}

	if (discriminatorKey == null) {
		throw Exception("All sealed classes must have a JsonDiscriminatorKey annotation. Failed at: $descriptor")
	}

	return ItemSchema.EnumSchema(discriminatorKey, contentKey, variants)
}

@OptIn(ExperimentalSerializationApi::class)
private fun getListSchema(descriptor: SerialDescriptor): ItemSchema {
	if (descriptor.elementsCount != 1) {
		throw Exception("A list must have exactly on child element")
	}

	return ItemSchema.ListSchema(option = getItemSchema(descriptor.getElementDescriptor(0)))
}

@OptIn(ExperimentalSerializationApi::class)
private fun getClassSchema(descriptor: SerialDescriptor): ItemSchema {
	val items = mutableListOf<StructPropertySchema>()

	for (annotation in descriptor.annotations) {
		if (annotation is IsColor) return ItemSchema.ColorSchema
	}

	for (childIndex in 0..<descriptor.elementsCount) {
		val child = descriptor.getElementDescriptor(childIndex)
		val annotations = descriptor.getElementAnnotations(childIndex)

		items.add(
			StructPropertySchema(
				name = descriptor.getElementName(childIndex),
				type = getItemSchema(child, annotations),
				description = getDescription(annotations),
				optional = child.isNullable
			)
		)
	}

	return ItemSchema.StructSchema(properties = items)
}

private fun getDescription(annotations: List<Annotation>): String? {
	for (annotation in annotations) {
		if (annotation is Description) {
			return annotation.content
		}
	}

	return null
}

@OptIn(ExperimentalSerializationApi::class)
private fun getTopLevelVariant(klass: KClass<*>): String? {
	val descriptor = serialDescriptor(klass.createType())

	for (annotation in descriptor.annotations) {
		if (annotation is ObjectReference) {
			return serialDescriptor(
				annotation.expectedTopLevelVariant.createType()
			).serialName
		}
	}

	return null
}

@Serializable
data class Schema(@SerialName("object") val obj: ItemSchema) {
	val version = "0.1"

	@SerialName("initial_objects")
	val initialObjects = listOf(
		InitialObject(
			id = "theme_default",
			description = "The theme that will be applied by default to all UI elements",
			expectedTopLevelVariant = getTopLevelVariant(Theme::class)
		),
		InitialObject(
			id = "layout_default",
			description = "The layout that will wrap everything",
			expectedTopLevelVariant = getTopLevelVariant(Theme::class)
		),
	)
}

@Serializable
data class InitialObject(
	val id: String,
	val description: String,
	@SerialName("expected_top_level_variant") val expectedTopLevelVariant: String?,
)

@OptIn(ExperimentalSerializationApi::class)
@JsonClassDiscriminator("$")
@Serializable
sealed class ItemSchema {
	@Serializable
	@SerialName("struct")
	data class StructSchema(
		val properties: List<StructPropertySchema>
	) : ItemSchema()

	@Serializable
	@SerialName("enum")
	data class EnumSchema(
		@SerialName("discriminator_key") val discriminatorKey: String,
		@SerialName("content_key") val contentKey: String?,
		val variants: List<EnumVariantSchema>
	) : ItemSchema()

	@Serializable
	@SerialName("string")
	data object StringSchema : ItemSchema()

	@Serializable
	@SerialName("number")
	data object NumberSchema : ItemSchema()

	@Serializable
	@SerialName("boolean")
	data object BooleanSchema : ItemSchema()

	@Serializable
	@SerialName("color")
	data object ColorSchema : ItemSchema()

	@Serializable
	@SerialName("list")
	data class ListSchema(
		val option: ItemSchema
	) : ItemSchema()

	@Serializable
	@SerialName("reference")
	data class ReferenceSchema(
		val expectedTopLevelVariant: String?
	) : ItemSchema()
}

@Serializable
data class StructPropertySchema(
	val name: String,
	val description: String?,
	val type: ItemSchema,
	val optional: Boolean
)

@Serializable
data class EnumVariantSchema(val name: String, val description: String?, val type: ItemSchema)
