package cmd

import (
	"log/slog"
	"os"

	"github.com/lmittmann/tint"
	"github.com/mattn/go-colorable"
	"github.com/mattn/go-isatty"
)

func setupLogger(isVerbose bool, isDebug bool) {
	level := slog.LevelInfo

	if isDebug {
		isVerbose = true
		level = slog.LevelDebug
	}

	if !isVerbose {
		return
	}

	w := os.Stderr
	handler := tint.NewHandler(colorable.NewColorable(w), &tint.Options{
		NoColor:   !isatty.IsTerminal(w.Fd()),
		AddSource: isDebug,
		Level:     level,
	})

	slog.SetDefault(slog.New(handler))
}
