package cmd

import (
	"github.com/charmbracelet/huh"
	"github.com/fatih/color"

	"github.com/radical-ui/objection/cmd/frontend"
	"github.com/radical-ui/objection/cmd/project_config"
)

func runWithErrorHandling() {
	if err := run(); err != nil {
		if err != huh.ErrUserAborted {
			color.Red("✗ %s", err)
		}
	}
}

func run() error {
	// we initialize the logger first becuase we'll want other system to be able to use it
	setupLogger(globalArgs.verbose, globalArgs.debug)

	// suppliedProjectConfigFile is allowed to be empty
	projectConfig, err := project_config.NewProjectConfig(globalArgs.config, globalArgs.neverSave)
	if err != nil {
		return err
	}

	// we will prompt the user for a frontend if they did not pass one
	// we're even gonna be extra nice and save it to their config if it is new and they gave us an alias
	frontendExpression := globalArgs.frontend
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

	if frontendManager.ShouldDownload() {
		if err := runTask("Downloading frontend...", "Downloaded frontend", frontendManager.Download); err != nil {
			return err
		}
	}

	// if they have a remote repo for the frontend, we must get a rev
	if len(frontendInfo.RemoteLocation) != 0 && len(frontendInfo.RemoteRev) == 0 {
		revInfo, err := frontendManager.GetRevInfo()
		if err != nil {
			return err
		}

		rev, err := selectRev(revInfo)
		if err != nil {
			return err
		}

		frontendInfo.RemoteRev = rev

		if len(frontendInfo.Alias) != 0 {
			projectConfig.SetFrontendRev(frontendInfo.Alias, frontendInfo.RemoteRev)
		}
	}

	// Before we try to get the config, we need to make sure the rev is applied. This is because the config can change with the different revisions
	frontendManager.EnsureRevIsApplied()

	// build the frontend config
	// prepare to configure
	// configure

	return nil
}
