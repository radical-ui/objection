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

func (self *ProjectConfig) ResolveFrontend(expression string) (*FrontendInfo, error) {
	for _, frontend := range self.localConfig.frontends {
		if frontend.alias == expression || frontend.location == expression {
			if frontend.location == expression {
				slog.Warn(fmt.Sprintf("Assuming frontend with alias '%s' because the location matched. In the future, refer to this frontend by it's alias", frontend.alias))
			}

			info, err := frontend.getInfo()
			if err != nil {
				return nil, err
			}

			return &info, nil
		}
	}

	return nil, nil
}

func (self *frontendDef) getInfo() (FrontendInfo, error) {
	var localLocation, remoteLocation, remoteRev string

	info, err := os.Stat(self.location)
	if err != nil && info.IsDir() {
		localLocation = self.location
	} else {
		remoteLocation = self.location
	}

	remoteRev = self.rev

	return FrontendInfo{localLocation, remoteLocation, remoteRev, self.configuration}, nil
}
