package cmd

import (
	"github.com/cqroot/prompt"
	// "github.com/cqroot/prompt/input"
)

func selectFrontend() (string, error) {
	return prompt.New().Ask("Frontend:").Input("git@github.com:radical-ui/ipage")
}
