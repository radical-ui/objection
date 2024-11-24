package project_config

import (
	"log/slog"
	"os"

	"github.com/hashicorp/hcl/v2/hclsimple"
)

const (
	defaultConfigPath = "objection.hcl"
)

type ProjectConfig struct {
	path        string
	neverSave   bool
	localConfig rootDef
}

type rootDef struct {
	Frontends []*frontendDef `hcl:"frontend,block"`
}

type frontendDef struct {
	Alias         string            `hcl:"alias,label"`
	Location      string            `hcl:"location,optional"`
	Rev           string            `hcl:"rev,optional"`
	BindingsPath  string            `hcl:"bindings_path,optional"`
	Configuration map[string]string `hcl:"configuration,optional"`
}

// Create a new project config. `initialLocation` is allowed to be empty, but if it is not, the file it points
// to will be read and parsed, otherwise, an empty config will be used.
//
// If `initialLocation` does not exist, an error will be thrown, so leave it empty if it doesn't exist. It will
// default to `objection.hcl` for mutations. Set `neverSave` to `true` to prevent the writing to (or potential
// creation of) the config file.
func NewProjectConfig(initialLocation string, neverSave bool) (*ProjectConfig, error) {
	location := provideLocationDefault(initialLocation)
	if len(location) == 0 {
		slog.Info("assuming default project config path, but it does not exist", "default_config", defaultConfigPath)

		return &ProjectConfig{path: defaultConfigPath}, nil
	}

	config := ProjectConfig{path: location}

	slog.Info("reading project config", "path", defaultConfigPath)

	if err := hclsimple.DecodeFile(location, nil, &config.localConfig); err != nil {
		return nil, err
	}

	slog.Debug("parsed project config", "data", config.localConfig)

	return &config, nil
}

func (self *ProjectConfig) GetExistingFrontends() []string {
	var frontends []string

	for _, def := range self.localConfig.Frontends {
		frontends = append(frontends, def.Alias)
	}

	return frontends
}

func provideLocationDefault(initialLocation string) string {
	if len(initialLocation) != 0 {
		return initialLocation
	}

	_, err := os.Stat(defaultConfigPath)
	if err != nil {
		return ""
	}

	return defaultConfigPath
}
