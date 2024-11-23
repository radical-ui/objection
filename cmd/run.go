package cmd

import (
	"github.com/radical-ui/objection/cmd/frontend"
	"github.com/radical-ui/objection/cmd/project_config"
)

func run() error {
	// suppliedProjectConfigFile is allowed to be empty
	projectConfig, err := project_config.NewProjectConfig(suppliedProjectConfigFile, suppliedProjectConfigNeverSave)
	if err != nil {
		return err
	}

	// we will prompt the user for a frontend if they did not pass one
	// we're even gonna be extra nice and save it to their config if it is new and they gave us an alias
	frontendExpression := suppliedFrontend
	if len(frontendExpression) == 0 {
		selectedFrontend, err := selectFrontend(projectConfig.GetExistingFrontends())
		if err != nil {
			return err
		}

		if len(selectedFrontend.alias) != 0 {
			frontendExpression = selectedFrontend.alias
		} else {
			frontendExpression = selectedFrontend.location
		}

		if selectedFrontend.isNew && len(selectedFrontend.alias) != 0 {
			if err := projectConfig.AddFrontend(selectedFrontend.location, selectedFrontend.alias); err != nil {
				return err
			}
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

	// we create a lockfile so that another instance of this tool cannot write to the cache at the same time as us and bork stuff
	// the lockfile is scoped by the frontend location, so it'll only block another process if they used the same frontend
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
