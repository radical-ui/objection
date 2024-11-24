package schema

import (
	"fmt"
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
	self.writeFunc(
		cases.ToPascal(object.Name),
		self.frontendName,
		func() {
			self.writeCommaSeperatedList([]func(){
				func() {
					self.text.WriteString("attributes string")
				},
				func() {
					self.text.WriteString("children func()")
				},
			})
		},
		"",
		func() {
		},
	)
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

func (self *generator) writeJsonStructItem(name string, ty string) {
	// does this: name ty `json:"name"`

	self.text.WriteString(cases.ToPascal(name))
	self.text.WriteRune(' ')
	self.text.WriteString(ty)
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
