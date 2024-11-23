package project_config

import (
	"fmt"
	"log/slog"
	"os"
)

type FrontendInfo struct {
	LocalLocation  string
	RemoteLocation string
	RemoteRev      string
	Configuration  map[string]string
}

// Resolve the supplied frontend into some frontend info. `expression` must not be empty
func (self *ProjectConfig) ResolveFrontend(expression string) (*FrontendInfo, error) {
	for _, frontend := range self.localConfig.Frontends {
		if frontend.Alias == expression || frontend.Location == expression {
			if frontend.Location == expression {
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

	return FrontendInfo{localLocation, remoteLocation, remoteRev, self.Configuration}, nil
}
