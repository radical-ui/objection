package project_config

import (
	"errors"
	"fmt"
	"os"

	"github.com/hashicorp/hcl/v2/gohcl"
	"github.com/hashicorp/hcl/v2/hclwrite"
)

func (self *ProjectConfig) AddFrontend(location string, name string) error {
	frontend := frontendDef{
		Location: location,
		Alias:    name,
	}
	self.localConfig.Frontends = append(self.localConfig.Frontends, &frontend)

	self.save()

	return nil
}

func (self *ProjectConfig) SetFrontendRev(alias string, rev string) error {
	def := self.getFrontendByAlias(alias)
	if def == nil {
		return fmt.Errorf("Frontend '%s' does not exist", alias)
	}

	def.Rev = rev

	self.save()

	return nil
}

func (self *ProjectConfig) getFrontendByAlias(alias string) *frontendDef {
	for _, def := range self.localConfig.Frontends {
		if def.Alias == alias {
			return def
		}
	}

	return nil
}

func (self *ProjectConfig) save() error {
	if self.neverSave {
		return nil
	}

	file := hclwrite.NewEmptyFile()
	gohcl.EncodeIntoBody(&self.localConfig, file.Body())

	if err := os.WriteFile(self.path, file.Bytes(), os.ModePerm); err != nil {
		return errors.Join(errors.New(fmt.Sprintf("Failed to write modified config to %s", self.path)), err)
	}

	return nil
}
