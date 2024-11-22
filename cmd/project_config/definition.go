package project_config

import (
	"os"

	"github.com/hashicorp/hcl/v2/hclsimple"
)

type ProjectConfig struct {
	neverSave   bool
	localConfig rootDef
}

type rootDef struct {
	frontends []frontendDef `hcl:"frontend,block"`
}

type frontendDef struct {
	alias         string            `hcl:"alias,label"`
	location      string            `hcl:"location,optional"`
	rev           string            `hcl:"rev,optional"`
	configuration map[string]string `hcl:"configuration,optional"`
}

func NewProjectConfig(initialLocation string, neverSave bool) (ProjectConfig, error) {
	config := ProjectConfig{}

	location := provideLocationDefault(initialLocation)
	if len(location) == 0 {
		return config, nil
	}

	if err := hclsimple.DecodeFile(location, nil, &config.localConfig); err != nil {
		return ProjectConfig{}, err
	}

	return config, nil
}

func provideLocationDefault(initialLocation string) string {
	if len(initialLocation) == 0 {
		return initialLocation
	}

	_, err := os.Stat("objection.hcl")
	if err != nil {
		return ""
	}

	return "objection.hcl"
}
