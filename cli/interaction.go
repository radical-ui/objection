package main

import (
	"errors"
	"fmt"

	"github.com/charmbracelet/huh"
	"github.com/charmbracelet/huh/spinner"
	"github.com/fatih/color"
	"github.com/radical-ui/objection/cli/frontend"
)

type selectableFrontend struct {
	isNew    bool
	location string
	alias    string
}

// Select a frontend from `existingFrontend` or return a new frontend. If an existing frontend is
// returned, `location` will always be empty. If a new frontend is returned, `alias` might be empty.
func selectFrontend(existingFrontends []string) (selectableFrontend, error) {
	var selectedFrontend int
	newFrontend := selectableFrontend{isNew: true}

	if len(existingFrontends) > 0 {
		options := make([]huh.Option[int], 0)

		for index, selection := range existingFrontends {
			options = append(options, huh.NewOption(selection, index))
		}

		options = append(options, huh.NewOption("Use a different frontend", len(existingFrontends)))

		input := huh.NewSelect[int]().
			Title("Select a frontend").
			Description("Bypass this in the future by specifing the --frontend option").
			Options(options...).
			Value(&selectedFrontend)

		if err := input.Run(); err != nil {
			return selectableFrontend{}, err
		}
	}

	if selectedFrontend < len(existingFrontends) {
		return selectableFrontend{alias: existingFrontends[selectedFrontend]}, nil
	}

	form := huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Provide a frontend").
				Description("This can be a git url (ssh or http[s]), or a path to a directory on your file system").
				Validate(func(value string) error {
					if len(value) != 0 {
						return nil
					}

					return errors.New("Frontend must not be empty")
				}).
				Value(&newFrontend.location),
			huh.NewInput().
				Title("Alias").
				Description("If you give us an alias, we will save it to your config so you don't have to type the url each time").
				Value(&newFrontend.alias),
		),
	)

	if err := form.Run(); err != nil {
		return selectableFrontend{}, err
	}

	return newFrontend, nil
}

func selectRev(info frontend.RevInfo) (string, error) {
	const TAG = 0
	const RECENT_COMMIT = 1
	const DEFULT = 2
	const CUSTOM = 3

	var selectedOption int
	var selectedRev string
	var options []huh.Option[int]

	if len(info.Tags) != 0 {
		options = append(options, huh.NewOption(fmt.Sprintf("A tag (%d available)", len(info.Tags)), TAG))
	}

	options = append(options, huh.NewOption(fmt.Sprintf("One of the most recent commit %d commits", len(info.Commits)), RECENT_COMMIT))
	options = append(options, huh.NewOption(fmt.Sprintf("The default branch (%s)", info.DefaultRev), DEFULT))
	options = append(options, huh.NewOption("Enter a custom rev", CUSTOM))

	initialSelector := huh.NewSelect[int]().
		Title("Select a frontend revision").
		Description("You can skip this in the future by passing a --rev flag or specifying a value in the config").
		Options(options...).
		Value(&selectedOption)

	if err := initialSelector.Run(); err != nil {
		return "", err
	}

	if selectedOption == TAG {
		var options []huh.Option[string]

		for _, tag := range info.Tags {
			options = append(options, huh.NewOption(tag, tag))
		}

		tagSelector := huh.NewSelect[string]().
			Title("Select a tag").
			Options(options...).
			Value(&selectedRev)

		if err := tagSelector.Run(); err != nil {
			return "", err
		}
	}

	if selectedOption == RECENT_COMMIT {
		var options []huh.Option[string]

		for _, commit := range info.Commits {
			options = append(options, huh.NewOption(fmt.Sprintf("%s: %s", commit.Hash, commit.Message), commit.Hash))
		}

		commitSelector := huh.NewSelect[string]().
			Title("Select a recent commit").
			Options(options...).
			Value(&selectedRev)

		if err := commitSelector.Run(); err != nil {
			return "", err
		}
	}

	if selectedOption == DEFULT {
		selectedRev = info.DefaultRev
	}

	if selectedOption == CUSTOM {
		customInput := huh.NewInput().
			Title("Provide a rev").
			Value(&selectedRev).
			Validate(func(input string) error {
				if len(input) == 0 {
					return errors.New("Rev must not be empty")
				}

				return nil
			})

		if err := customInput.Run(); err != nil {
			return "", err
		}
	}

	return selectedRev, nil
}

func selectBindingsPath(suggestion string) (string, error) {
	var value string

	input := huh.NewInput().
		Title("Select bindings directory").
		Description("The golang bindings will be written to a `mod.go` file in this directory.").
		Placeholder(suggestion).
		Value(&value)

	if err := input.Run(); err != nil {
		return "", nil
	}

	if len(value) == 0 {
		return suggestion, nil
	}

	return value, nil
}

func runTask(loadingText string, successText string, task func() error) error {
	var err error
	action := func() {
		err = task()
	}

	spinner.New().Title(loadingText).Action(action).Run()

	if err != nil {
		return err
	}

	color.Green("✔ %s", successText)
	return nil
}
