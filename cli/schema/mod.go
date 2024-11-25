package schema

import (
	"encoding/json"
	"errors"
	"fmt"
	"go/format"
	"log/slog"
	"os"
)

type Schema struct {
	def schemaDef
}

type schemaDef struct {
	Objects []objectDef `json:"objects"`
}

type objectDef struct {
	Name       string       `json:"name"`
	Attributes []schemaType `json:"attributes"`
}

type schemaType struct {
	Kind             string    `json:"$"`
	DiscriminatorKey string    `json:"discriminator_key,omitempty"`
	ContentKey       string    `json:"content_key,omitempty"`
	Variants         []itemDef `json:"variants,omitempty"`
	Properties       []itemDef `json:"properties,omitempty"`
}

type itemDef struct {
	Name        string     `json:"name"`
	Description string     `json:"description"`
	SchemaType  schemaType `json:"type"`
}

func NewSchema(path string) (*Schema, error) {
	var def schemaDef

	slog.Info("reading frontend schema file", "path", path)

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, errors.Join(fmt.Errorf("frontend schema file '%s' could not be read", path), err)
	}

	json.Unmarshal(data, &def)

	slog.Debug("parsed schema file", "path", path, "data", def)

	return &Schema{def}, nil
}

func (self *Schema) GenBindings(frontendName string) ([]byte, error) {
	slog.Info("generating frontend bindings", "frontend", frontendName, "schema", self.def)
	generator := newGenerator(frontendName)

	generator.writeSchema(&self.def)
	text := generator.text.String()
	slog.Debug("bindings were generated", "code", text)

	slog.Info("formatting parsed code")
	res, err := format.Source([]byte(text))
	if err != nil {
		return make([]byte, 0), errors.Join(fmt.Errorf("failed to format generated code; this is a bug"), err)
	}

	return res, nil
}
