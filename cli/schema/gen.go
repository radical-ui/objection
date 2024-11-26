package schema

import (
	"fmt"
	"log/slog"
	"strings"

	"github.com/rossmacarthur/cases"
)

type generator struct {
	frontendName string
	text         strings.Builder
}

func newGenerator(frontendName string) *generator {
	return &generator{frontendName: cases.ToPascal(frontendName)}
}

func (self *generator) writeSchema(schema *schemaDef) {
	self.writePackageLabel(self.frontendName)

	self.writeImportBlock(func() {
		self.writeStringLiteral("github.com/radical-ui/objection")
	})

	self.writeStruct(self.frontendName, func() {
		self.text.WriteString("frontend *objection.Frontend")
	})

	self.writeFunc(
		fmt.Sprintf("New%s", self.frontendName),
		"",
		func() {
			self.text.WriteString("frontend *objection.Frontend")
		},
		self.frontendName,
		func() {
			self.writeReturnStatement(func() {
				self.writeStructConstruction(self.frontendName, func() {
					self.text.WriteString("frontend,")
				})
			})
		},
	)

	for index := range schema.Objects {
		self.writeObjectMethod(&schema.Objects[index])
	}
}

func (self *generator) writeObjectMethod(object *objectDef) {
	var returnFunc func()

	self.writeFunc(
		cases.ToPascal(object.Name),
		self.frontendName,
		func() {
			self.writeCommaSeperatedList([]func(){
				func() {
					self.text.WriteString("attributes ")
					returnFunc = self.writeSchemaType([]string{object.Name}, &object.Attributes)
				},
				func() {
					self.text.WriteString("children func()")
				},
			})
		},
		"",
		func() {
			self.text.WriteString("self.frontend.StartNewObject(")
			self.writeStringLiteral(object.Name)
			self.text.WriteString(")\n")

			self.text.WriteString("self.frontend.SetAttributes(attributes)\n")
			self.text.WriteString("self.frontend.CurrentChildrenFunc = children\n")
			self.text.WriteString("children()\n")
			self.text.WriteString("self.frontend.FinishObject()")
		},
	)

	returnFunc()
}

func (self *generator) writeSchemaType(stack []string, ty *schemaType) func() {
	var returnFunc func()

	writeWithNillControl := func(ty *schemaType, key string) {
		if ty.Child != nil {
			returnFunc = self.writeSchemaType(stack, ty.Child)
		} else {
			slog.Error("found a nill value in schema where it was not supposed to exist", "key", key, "stack", stack)
			returnFunc = self.writeSchemaType(stack, &schemaType{})
		}
	}

	switch ty.Kind {
	case "string":
		self.text.WriteString("string")
	case "number":
		self.text.WriteString("float64")
	case "boolean":
		self.text.WriteString("bool")
	case "list":
		self.text.WriteString("[]")
		writeWithNillControl(ty.Item, "item")
	case "color":
		self.text.WriteString("objection.Color")
	case "binding":
		self.text.WriteString("objection.Binding[")
		writeWithNillControl(ty.Child, "child")
		self.text.WriteRune(']')
	case "optional":
		self.text.WriteRune('*')
		writeWithNillControl(ty.Child, "child")
	case "struct":
		returnFunc = self.writeStructSchemaType(stack, ty)
	case "enum":
		returnFunc = self.writeEnumSchemaType(stack, ty)
	default:
		slog.Warn("found a schema kind that was not supposed to exist", "kind", ty.Kind, "stack", stack)
	}

	if returnFunc == nil {
		return func() {}
	}

	return returnFunc
}

func (self *generator) writeStructSchemaType(stack []string, ty *schemaType) func() {
	name := stackToPublicName(stack)

	self.text.WriteString(name)

	return func() {
		self.writeStruct(name, func() {
			for _, property := range ty.Properties {
				self.writeComment(property.Description)
				self.writeJsonStructItem(property.Name, func() {
					self.writeSchemaType(append(stack, property.Name), &property.SchemaType)
				})
			}
		})
	}
}

func (self *generator) writeEnumSchemaType(stack []string, ty *schemaType) func() {
	name := stackToPublicName(stack)
	self.text.WriteString(name)

	markerFunc := fmt.Sprintf("is%s()", name)

	return func() {
		var additionalBuilders []func()

		self.writeInterface(name, func() {
			self.text.WriteString(markerFunc)
		})

		for _, variant := range ty.Variants {
			variantStack := append(stack, variant.Name)
			variantName := stackToPublicName(variantStack)

			self.writeComment(variant.Description)
			self.writeStruct(variantName, func() {
				additionalBuilders = append(additionalBuilders, self.writeSchemaType(variantStack, &variant.SchemaType))
			})

			self.writeFunc(markerFunc, variantName, func() {}, "", func() {})
		}
	}
}

func (self *generator) writeImportBlock(content func()) {
	self.text.WriteString("import (")
	content()
	self.text.WriteString(")\n\n")
}

func (self *generator) writePackageLabel(name string) {
	self.text.WriteString("package ")
	self.text.WriteString(cases.ToSnake(name))
	self.text.WriteString("\n\n")
}

func (self *generator) writeReturnStatement(statement func()) {
	self.text.WriteString("return ")
	statement()
	self.text.WriteRune('\n')
}

func (self *generator) writeStructConstruction(name string, body func()) {
	self.text.WriteString(name)
	self.text.WriteString("{\n")

	body()

	self.text.WriteString("\n}")
}

func (self *generator) writeStringLiteral(content string) {
	self.text.WriteRune('"')
	self.text.WriteString(strings.ReplaceAll(content, "\"", "\\\""))
	self.text.WriteRune('"')
}

func (self *generator) writeJsonStructItem(name string, ty func()) {
	// does this: name ty `json:"name"`

	self.text.WriteString(cases.ToPascal(name))
	self.text.WriteRune(' ')
	ty()
	self.text.WriteString(" `json:\"")
	self.text.WriteString(name)
	self.text.WriteString("\"`\n")
}

func (self *generator) writeCommaSeperatedList(items []func()) {
	for index, fn := range items {
		if index != 0 {
			self.text.WriteString(", ")
		}

		fn()
	}
}

func (self *generator) writeStruct(name string, body func()) {
	self.text.WriteString("type ")
	self.text.WriteString(name)
	self.text.WriteString(" struct {\n")

	body()

	self.text.WriteString("\n}\n\n")
}

func (self *generator) writeInterface(name string, body func()) {
	self.text.WriteString("type ")
	self.text.WriteString(name)
	self.text.WriteString(" interface {\n")

	body()

	self.text.WriteString("\n}\n\n")
}

func (self *generator) writeFunc(name string, ptrTypeAssociation string, args func(), rtr string, body func()) {
	self.text.WriteString("func ")

	if len(ptrTypeAssociation) != 0 {
		self.text.WriteString("(self *")
		self.text.WriteString(ptrTypeAssociation)
		self.text.WriteString(") ")
	}

	self.text.WriteString(name)

	self.text.WriteString("(")
	args()
	self.text.WriteString(")")

	if len(rtr) != 0 {
		self.text.WriteRune(' ')
		self.text.WriteString(rtr)
	}

	self.text.WriteString(" {\n")
	body()
	self.text.WriteString("\n}\n\n")
}

func (self *generator) writeComment(comment string) {
	if len(comment) == 0 {
		return
	}

	self.text.WriteString("// ")

	for _, char := range comment {
		if char == '\n' {
			self.text.WriteString("\n// ")
		} else {
			self.text.WriteRune(char)
		}
	}

	self.text.WriteRune('\n')
}

func stackToPublicName(stack []string) string {
	writer := strings.Builder{}

	for _, item := range stack {
		writer.WriteString(cases.ToPascal(item))
	}

	return writer.String()
}
