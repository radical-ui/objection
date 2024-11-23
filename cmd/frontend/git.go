package frontend

import (
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path"
	"strings"

	"github.com/otiai10/copy"
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
	// self.cache.aquireLock(remote)
	// defer self.cache.releaseLock(remote)

	if err := self.runTransparentCommand("clone", self.remote, self.cache.DownloadDir); err != nil {
		return errors.Join(errors.New(fmt.Sprintf("Failed to clone %s", self.remote)), err)
	}

	// we never want to use pagers. Of course we could run this command when we need to get the tags, but it makes a lot of
	// sense to never have the repo in a state where a pager is available
	if err := self.runTransparentCommand("config", "pager.tag", "false"); err != nil {
		return errors.Join(errors.New(fmt.Sprintf("Failed to configure the git to not use a pager for tags")), err)
	}

	return nil
}

func (self *git) listAllTags(remote string) ([]string, error) {
	// TODO checkout the
	data, err := self.runCapturedCommand(remote, "tags")
	if err != nil {
		return make([]string, 0), err
	}

	return strings.Split(data, "\n"), nil
}

func (self *git) copyDownloadedRepoForConfiguration(rev string) error {
	// self.cache.aquireLock(remote)
	// defer self.cache.releaseLock(remote)

	if err := self.runTransparentCommand("checkout", rev); err != nil {
		return err
	}

	if err := gitFriendlyCopy(self.cache.DownloadDir, self.cache.ConfigureDir); err != nil {
		return err
	}

	// if err := self.runTransparentCommand(remote, "checkout", "-"); err != nil {
	// 	return err
	// }

	return nil
}

func (self *git) copyLocalFolderForConfiguration(localFolder string) error {
	if err := gitFriendlyCopy(localFolder, self.cache.ConfigureDir); err != nil {
		return err
	}

	return nil
}

func (self *git) runTransparentCommand(args ...string) error {
	cmd := exec.Command(self.path, args...)
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout
	cmd.Stdin = os.Stdin
	cmd.Dir = self.cache.DownloadDir

	if err := cmd.Run(); err != nil {
		return err
	}

	return nil
}

func (self *git) runCapturedCommand(args ...string) (string, error) {
	cmd := exec.Command(self.path, args...)
	cmd.Dir = self.cache.DownloadDir

	out, err := cmd.Output()
	if err != nil {
		return "", err
	}

	return string(out), nil
}

func gitFriendlyCopy(src string, dest string) error {
	return copy.Copy(src, dest, copy.Options{
		Skip: func(srcinfo os.FileInfo, src, dest string) (bool, error) {
			if srcinfo.IsDir() && path.Base(src) == ".git" {
				return true, nil
			}

			return false, nil
		},
	})
}
