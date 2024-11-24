package project_config

import (
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path"
)

type FrontendInfo struct {
	LocalLocation  string
	RemoteLocation string
	RemoteRev      string
	Configuration  map[string]string
	Alias          string
	BindingsPath   string
}

func (self *FrontendInfo) GetName() string {
	if len(self.Alias) != 0 {
		return self.Alias
	}

	if len(self.LocalLocation) != 0 {
		return path.Base(self.LocalLocation)
	}

	return path.Base(self.RemoteLocation)
}

func (self *FrontendInfo) WriteBindings(data []byte) error {
	file := path.Join(self.BindingsPath, "mod.go")
	slog.Info("writing bindings", "path", self.BindingsPath)

	if err := os.WriteFile(file, data, os.ModePerm); err != nil {
		slog.Error("failed to write bindings because something doesn't exist")

		if os.IsNotExist(err) {
			slog.Info("creating directory to retry failing binding write")

			dir := self.BindingsPath
			if err := os.MkdirAll(dir, os.ModePerm); err != nil {
				return errors.Join(fmt.Errorf("failed to create directory '%s' after a direct bindings write failed", dir), err)
			}

			if err := os.WriteFile(file, data, os.ModePerm); err != nil {
				return errors.Join(fmt.Errorf("failed to write bindings to '%s'", self.BindingsPath), err)
			}

			return nil
		}

		return errors.Join(fmt.Errorf("failed to write bindings to '%s'", self.BindingsPath), err)
	}

	return nil
}

// Resolve the supplied frontend into some frontend info. `expression` must not be empty
func (self *ProjectConfig) ResolveFrontend(expression string) (*FrontendInfo, error) {
	for _, frontend := range self.localConfig.Frontends {
		if frontend.Alias == expression || frontend.Location == expression {
			if frontend.Alias != expression && frontend.Location == expression {
				slog.Warn(fmt.Sprintf("Assuming frontend with alias '%s' because the location matched. In the future, refer to this frontend by it's alias", frontend.Alias))
			}

			info, err := frontend.getInfo()
			if err != nil {
				return nil, err
			}

			return &info, nil
		}
	}

	def := frontendDef{Location: expression}

	info, err := def.getInfo()
	if err != nil {
		return nil, err
	}

	return &info, nil
}

func (self *frontendDef) getInfo() (FrontendInfo, error) {
	var localLocation, remoteLocation, remoteRev string

	info, err := os.Stat(self.Location)
	if err == nil && info.IsDir() {
		localLocation = self.Location
	} else {
		remoteLocation = self.Location
	}

	remoteRev = self.Rev

	return FrontendInfo{localLocation, remoteLocation, remoteRev, self.Configuration, self.Alias, self.BindingsPath}, nil
}
