package cmd

import (
	"log/slog"

	"github.com/spf13/cobra"
)

var newCmd = &cobra.Command{
	Use:   "new",
	Short: "A brief description of your command",
	Run: func(cmd *cobra.Command, args []string) {
		if err := run(); err != nil {
			slog.Error(err.Error())
			return
		}
	},
}

func init() {
	rootCmd.AddCommand(newCmd)
}
