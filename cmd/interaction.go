package cmd

import (
	// "github.com/cqroot/prompt"
	// "github.com/cqroot/prompt/input"
	"errors"

	"github.com/charmbracelet/huh"
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
		options := make([]huh.Option[int], len(existingFrontends)+1)

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
