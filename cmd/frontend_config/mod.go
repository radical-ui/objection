package frontend_config

import (
	"log/slog"
	"path"

	"github.com/hashicorp/hcl/v2/hclsimple"
)

type FrontendConfig struct {
	configPath string
	config     configDef
}

type configDef struct {
	SchemaFile string       `hcl:"schema_file"`
	Configure  configureDef `hcl:"configure,block"`
}

type configureDef struct {
	Strings []stringDef `hcl:"string,block"`
	Colors  []colorDef  `hcl:"color,block"`
	Images  []imageDef  `hcl:"image,block"`
}

type stringDef struct {
	Key          string           `hcl:"key,label"`
	Replacements []replaceLineDef `hcl:"replace_line,block"`
	Renames      []renameDef      `hcl:"rename,block"`
}

type colorDef struct {
	Key     string           `hcl:"key,label"`
	Formats []colorFormatDef `hcl:"format,block"`
}

type colorFormatDef struct {
	Format       string           `hcl:"format,label"`
	Replacements []replaceLineDef `hcl:"replace_line,block"`
	Renames      []renameDef      `hcl:"rename,block"`
}

type renameDef struct {
	Path string `hcl:"path"`
	To   string `hcl:"to"`
}

type replaceLineDef struct {
	File  string `hcl:"file,optional"`
	Glob  string `hcl:"glob,optional"`
	Match string `hcl:"match"`
	Write string `hcl:"write"`
}

type imageDef struct {
	Key    string          `hcl:"key,label"`
	Writes []imageWriteDef `hcl:"write,block"`
}

type imageWriteDef struct {
	Path   string `hcl:"path,label"`
	Width  int    `hcl:"width"`
	Height int    `hcl:"height"`
	Format string `hcl:"format,optional"`
}

func NewFrontendConfig(path string) (*FrontendConfig, error) {
	var config configDef

	slog.Info("reading frontend config", "path", path)

	if err := hclsimple.DecodeFile(path, nil, &config); err != nil {
		return nil, err
	}

	return &FrontendConfig{path, config}, nil
}

func (self *FrontendConfig) GetSchemaFile() string {
	return path.Join(path.Dir(self.configPath), self.config.SchemaFile)
}
