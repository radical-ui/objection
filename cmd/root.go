package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

var (
	suppliedFrontend               string
	suppliedRev                    string
	suppliedProjectConfigFile      string
	suppliedProjectConfigNeverSave bool
)

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
	rootCmd.PersistentFlags().StringVarP(&suppliedProjectConfigFile, "config", "c", "", "Path to a HCL config file. Defaults to objection.hcl")
	rootCmd.PersistentFlags().StringVarP(&suppliedFrontend, "frontend", "f", "", "Git URL or local directory path for the frontend")
	rootCmd.PersistentFlags().StringVarP(&suppliedRev, "rev", "r", "", "Frontend Git revision to use")
	rootCmd.PersistentFlags().BoolVarP(&suppliedProjectConfigNeverSave, "never-save", "n", false, "Do not save frontend configurations")
}
