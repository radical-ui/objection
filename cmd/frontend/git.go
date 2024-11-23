package frontend

import (
	"bytes"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"strings"
)

type git struct {
	remote string
	cache  *cache
	path   string
}

func newGit(cache *cache, remote string) *git {
	path, ok := os.LookupEnv("OBJECTION_GIT_PATH")
	if !ok {
		path = "git"
	}

	return &git{remote, cache, path}
}

func (self *git) downloadRepo() error {
	if _, err := self.runCommand("clone", self.remote, self.cache.DownloadDir); err != nil {
		return errors.Join(errors.New(fmt.Sprintf("Failed to clone %s", self.remote)), err)
	}

	// we never want to use pagers. Of course we could run this command when we need to get the tags, but it makes a lot of
	// sense to never have the repo in a state where a pager is available
	if _, err := self.runCommand("config", "pager.tag", "false"); err != nil {
		return errors.Join(errors.New(fmt.Sprintf("Failed to configure the git to not use a pager for tags")), err)
	}

	return nil
}

func (self *git) listAllTags() ([]string, error) {
	data, err := self.runCommand("tag")
	if err != nil {
		return make([]string, 0), err
	}

	return breakCliOutput(data), nil
}

func (self *git) getDefaultRev() (string, error) {
	data, err := self.runCommand("rev-parse", "--abbrev-ref", "origin/HEAD")
	if err != nil {
		return "", err
	}

	return strings.TrimSuffix(string(data), "\n"), nil
}

type Commit struct {
	Hash    string
	Message string
}

func (self *git) getRecentCommits(count int) ([]Commit, error) {
	commits := make([]Commit, 0)

	data, err := self.runCommand("log", fmt.Sprintf("-%d", count), "--pretty=format:\"%h %s\"")
	if err != nil {
		return commits, err
	}

	for _, line := range breakCliOutput(data) {
		chunks := strings.SplitN(line, " ", 2)
		commits = append(commits, Commit{Hash: chunks[0], Message: chunks[1]})
	}

	return commits, nil
}

func (self *git) checkoutRev(rev string) error {
	if _, err := self.runCommand("checkout", rev); err != nil {
		return err
	}

	return nil
}

func (self *git) runCommand(args ...string) (string, error) {
	inner := func() (string, error) {
		var stdout bytes.Buffer
		var stderr bytes.Buffer

		cmd := exec.Command(self.path, args...)
		cmd.Dir = self.cache.DownloadDir
		cmd.Stdout = &stdout
		cmd.Stderr = &stderr

		slog.Info("Running git command", "args", args, "dir", self.cache.DownloadDir)

		if err := cmd.Run(); err != nil {
			stderrText := strings.TrimSpace(stderr.String())
			if len(stderrText) > 0 {
				return "", errors.Join(err, errors.New(stderrText))
			}

			return "", err
		}

		return stdout.String(), nil
	}

	out, err := inner()

	if err != nil {
		slog.Error("git command failed", "args", args, "dir", self.cache.DownloadDir)

		if os.IsNotExist(err) {
			slog.Warn("git command failed because something doesn't exist; creating dir and trying again", "dir", self.cache.DownloadDir)

			if err := os.MkdirAll(self.cache.DownloadDir, os.ModePerm); err != nil {
				return "", errors.Join(fmt.Errorf("Tried to create download dir at '%s'", self.cache.DownloadDir), err)
			}
			return inner()
		}

		return "", err
	}

	return out, nil
}

func breakCliOutput(input string) []string {
	items := strings.Split(input, "\n")
	var filteredItems []string

	for _, item := range items {
		if len(item) != 0 {
			filteredItems = append(filteredItems, item)
		}
	}

	return filteredItems
}
