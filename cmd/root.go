package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

type GlobalArgs struct {
	frontend  string
	rev       string
	config    string
	neverSave bool
	verbose   bool
	debug     bool
}

var globalArgs GlobalArgs

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
	rootCmd.PersistentFlags().StringVarP(&globalArgs.config, "config", "c", "", "Path to a HCL config file. Defaults to objection.hcl")
	rootCmd.PersistentFlags().StringVarP(&globalArgs.frontend, "frontend", "f", "", "Git URL or local directory path for the frontend")
	rootCmd.PersistentFlags().StringVarP(&globalArgs.rev, "rev", "r", "", "Frontend Git revision to use")
	rootCmd.PersistentFlags().BoolVarP(&globalArgs.neverSave, "never-save", "n", false, "Do not save frontend configurations")
	rootCmd.PersistentFlags().BoolVarP(&globalArgs.verbose, "verbose", "v", false, "Print debug information")
	rootCmd.PersistentFlags().BoolVarP(&globalArgs.debug, "debug", "d", false, "Print debug information")
}
