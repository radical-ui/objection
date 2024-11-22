package cmd

import (
	"github.com/radical-ui/objection/cmd/project_config"
)

type Frontend struct {
	info  project_config.FrontendInfo
	cache cache
}

func NewFrontend(info project_config.FrontendInfo) (Frontend, error) {
	cache, err := newCache()
	if err != nil {
		return Frontend{}, err
	}

	return Frontend{info, cache}, nil
}

func (self *Frontend) Download() {
	if len(self.info.LocalLocation) != 0 {
		return
	}

}
