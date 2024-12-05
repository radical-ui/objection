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
	Objects []ObjectDef `json:"objects"`
}

type ObjectDef struct {
	foo        bool
	Name       string     `json:"name"`
	Attributes SchemaType `json:"attributes"`
}

type SchemaType struct {
	Kind string `json:"$"` // string, number, boolean, list, color, binding, optional, struct, enum

	// Used when kind is enum
	DiscriminatorKey string `json:"discriminator_key,omitempty"`

	// Used when kind is enum
	ContentKey string `json:"content_key,omitempty"`

	// Used when kind is enum
	Variants []ItemDef `json:"variants,omitempty"`

	// Used when kind is struct
	Properties []ItemDef `json:"properties,omitempty"`

	// Used when kind is list
	Item *SchemaType `json:"item,omitempty"`

	// Used when kind is binding or optional
	Child *SchemaType `json:"child,omitempty"`
}

type ItemDef struct {
	Name        string     `json:"name"`
	Description string     `json:"description"`
	SchemaType  SchemaType `json:"type"`
}

func NewSchemaFromPath(path string) (*Schema, error) {
	var schema Schema

	slog.Info("reading frontend schema file", "path", path)

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, errors.Join(fmt.Errorf("frontend schema file '%s' could not be read", path), err)
	}

	json.Unmarshal(data, &schema)

	slog.Debug("parsed schema file", "path", path, "data", schema)

	return &schema, nil
}

func (self *Schema) GenBindings(frontendName string) ([]byte, error) {
	slog.Info("generating frontend bindings", "frontend", frontendName, "schema", self)
	generator := newGenerator(frontendName)

	generator.writeSchema(self)
	text := generator.text.String()
	slog.Debug("bindings were generated", "code", text)

	slog.Info("formatting parsed code")
	res, err := format.Source([]byte(text))
	if err != nil {
		return make([]byte, 0), errors.Join(fmt.Errorf("failed to format generated code; this is a bug"), err)
	}

	return res, nil
}
