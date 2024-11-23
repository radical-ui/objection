package cmd

import (
	"github.com/radical-ui/objection/cmd/frontend"
	"github.com/radical-ui/objection/cmd/project_config"
)

func run() error {
	projectConfig, err := project_config.NewProjectConfig(suppliedProjectConfigFile, suppliedProjectConfigNeverSave)
	if err != nil {
		return err
	}

	frontendExpression := suppliedFrontend
	if len(frontendExpression) == 0 {
		frontendExpression, err = selectFrontend()
		if err != nil {
			return err
		}
	}

	frontendInfo, err := projectConfig.ResolveFrontend(frontendExpression)
	if err != nil {
		return err
	}

	frontendManager, err := frontend.NewFrontendManager(frontendInfo)
	if err != nil {
		return err
	}

	if err := frontendManager.Lock(); err != nil {
		return err
	}
	defer frontendManager.ReleaseLock()

	if err := frontendManager.DownloadIfNecessary(); err != nil {
		return err
	}

	// add rev info if it does not exist
	// build the frontend config
	// prepare to configure
	// configure

	return nil
}
