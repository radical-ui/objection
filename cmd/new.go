package cmd

import (
	"github.com/spf13/cobra"
)

var newCmd = &cobra.Command{
	Use:   "new",
	Short: "A brief description of your command",
	Run: func(cmd *cobra.Command, args []string) {
		runWithErrorHandling()
	},
}

func init() {
	rootCmd.AddCommand(newCmd)
}
