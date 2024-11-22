package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var currentConfig config
var currentFrontend frontend

var rootCmd = &cobra.Command{
	Use:   "objection",
	Short: "Build server-first, highly-interactive, and beautiful web applications in Go",
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.PersistentFlags().StringVarP(&currentConfig.file, "config", "c", "objection.kdl", "Path to a KDL config file")
	rootCmd.PersistentFlags().StringVarP(&currentFrontend.pathOrUrl, "frontend", "f", "", "Git URL or local directory path for the frontend")
	rootCmd.PersistentFlags().StringVarP(&currentFrontend.rev, "rev", "r", "", "Frontend Git revision to use")
	rootCmd.PersistentFlags().BoolVarP(&currentConfig.neverSave, "never-save", "n", false, "Do not save frontend configurations")
}
