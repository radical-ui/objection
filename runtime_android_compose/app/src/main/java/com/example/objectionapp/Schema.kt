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
import kotlin.reflect.KType
import kotlin.reflect.full.createType

@ExperimentalSerializationApi
@SerialInfo
@Target(AnnotationTarget.PROPERTY)
annotation class Description(val content: String)

@ExperimentalSerializationApi
@SerialInfo
@Target(AnnotationTarget.PROPERTY)
annotation class ObjectReference(val expectedClass: KClass<*>)

@ExperimentalSerializationApi
@SerialInfo
@Target(AnnotationTarget.PROPERTY)
annotation class AnyObjectReference

@ExperimentalSerializationApi
fun getSchema(type: KType): Schema {
	return Schema(obj = getItemSchema(serialDescriptor(type)))
}

@ExperimentalSerializationApi
private fun getItemSchema(
	descriptor: SerialDescriptor,
	annotations: List<Any> = listOf()
): ItemSchema {
	when (descriptor.kind) {
		StructureKind.CLASS -> return getClassSchema(descriptor)
		PrimitiveKind.STRING -> return getStringSchema(annotations)
		PrimitiveKind.DOUBLE -> return ItemSchema.NumberSchema
		PrimitiveKind.FLOAT -> return ItemSchema.NumberSchema
		PrimitiveKind.INT -> return ItemSchema.NumberSchema
		PrimitiveKind.BOOLEAN -> return ItemSchema.BooleanSchema
		StructureKind.LIST -> return getListSchema(descriptor)
		PolymorphicKind.SEALED -> return getSealedSchema(descriptor)
		SerialKind.ENUM -> throw Exception("Use a sealed class with objects instead of an enum. Failed at: $descriptor")
		else -> {
			println(descriptor.kind)

			return getClassSchema(descriptor)
		}
	}
}

@ExperimentalSerializationApi
private fun getStringSchema(annotations: List<Any>): ItemSchema {
	for (annotation in annotations) {
		println(annotation)
		when (annotation) {
			is ObjectReference -> return ItemSchema.ReferenceSchema(
				expectedEnumName = serialDescriptor(
					annotation.expectedClass.createType()
				).serialName
			)

			is AnyObjectReference -> return ItemSchema.ReferenceSchema(expectedEnumName = null)
		}
	}

	return ItemSchema.StringSchema
}

@ExperimentalSerializationApi
private fun getSealedSchema(descriptor: SerialDescriptor): ItemSchema.EnumSchema {
	val variants = mutableListOf<EnumVariantSchema>()
	val child = descriptor.getElementDescriptor(1)
	var discriminatorKey: String? = null

	for (variant in child.elementDescriptors) {
		variants.add(
			EnumVariantSchema(
				name = variant.serialName,
				description = getDescription(listOf()), // FIXME
				type = getItemSchema(variant)
			)
		)
	}

	for (annotation in descriptor.annotations) {
		if (annotation is JsonClassDiscriminator) {
			discriminatorKey = annotation.discriminator
		}
	}

	if (discriminatorKey == null) {
		throw Exception("All sealed classes must have a JsonDiscriminatorKey annotation. Failed at: $descriptor")
	}

	return ItemSchema.EnumSchema(discriminatorKey, contentKey = null, variants)
}

@ExperimentalSerializationApi
private fun getListSchema(descriptor: SerialDescriptor): ItemSchema {
	if (descriptor.elementsCount != 1) {
		throw Exception("A list must have exactly on child element")
	}

	return ItemSchema.ListSchema(option = getItemSchema(descriptor.getElementDescriptor(0)))
}

@ExperimentalSerializationApi
private fun getClassSchema(descriptor: SerialDescriptor): ItemSchema.StructSchema {
	val items = mutableListOf<StructPropertySchema>()
	println(descriptor)

	for (childIndex in 0..<descriptor.elementsCount) {
		val child = descriptor.getElementDescriptor(childIndex)
		val annotations = descriptor.getElementAnnotations(childIndex)

		println("${descriptor.getElementName(childIndex)}: $annotations, $childIndex")
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

@ExperimentalSerializationApi
private fun getDescription(annotations: List<Any>): String? {
	for (annotation in annotations) {
		if (annotation is Description) {
			return annotation.content
		}
	}

	return null
}

@Serializable
@ExperimentalSerializationApi
data class Schema(@SerialName("object") val obj: ItemSchema)

@ExperimentalSerializationApi
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
		@SerialName("discriminator_key") val discriminatorKey: kotlin.String,
		@SerialName("content_key") val contentKey: kotlin.String?,
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
	@SerialName("list")
	data class ListSchema(
		val option: ItemSchema
	) : ItemSchema()

	@Serializable
	@SerialName("reference")
	data class ReferenceSchema(
		val expectedEnumName: String?
	) : ItemSchema()
}

@Serializable
@ExperimentalSerializationApi
data class StructPropertySchema(
	val name: String, val description: String?, val type: ItemSchema, val optional: Boolean
)

@Serializable
@ExperimentalSerializationApi
data class EnumVariantSchema(val name: String, val description: String?, val type: ItemSchema)
