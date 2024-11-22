package cmd

import (
	"crypto/md5"
	"encoding/hex"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"path"
	"time"

	"github.com/otiai10/copy"
)

var (
	downloadsDir = "frontend_downloads"
	locksDir     = "locks"
	configureDir = "frontend_configurations"
)

type cache struct {
	dir     string
	gitPath string
}

func newCache() (cache, error) {
	gitPath, ok := os.LookupEnv("OBJECTION_GIT_PATH")
	if !ok {
		gitPath = "git"
	}

	dirOverride, ok := os.LookupEnv("OBJECTION_CACHE_DIR")
	if ok {
		return cache{dirOverride, gitPath}, nil
	}

	userHome, ok := os.LookupEnv("HOME")
	if !ok {
		return cache{}, errors.New("Failed to find $HOME env var, as a step to locating a cache dir. Please provide this environment variable, or $OBJECTION_CACHE_DIR, which removes the need to get the home path")
	}

	return cache{path.Join(userHome, ".cache/objection"), gitPath}, nil
}

func hash(key string) string {
	hash := md5.Sum([]byte(key))
	return hex.EncodeToString(hash[:])
}

func (self *cache) downloadRepo(remote string, key string) error {
	dest := path.Join(self.dir, downloadsDir, hash(key))

	cmd := exec.Command(self.gitPath, "clone", remote, dest)
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout
	cmd.Stdin = os.Stdin

	if err := cmd.Run(); err != nil {
		return errors.Join(errors.New(fmt.Sprintf("Failed to clone %s", remote)), err)
	}

	return nil
}

func (self *cache) copyDownloadedRepoForConfigure(downloadedKey string, configureKey string) error {
	srcDir := path.Join(self.dir, downloadsDir, hash(downloadedKey))
	destDir := path.Join(self.dir, configureDir, hash(configureKey))

	if err := copy.Copy(srcDir, destDir); err != nil {
		return err
	}

	return nil
}

func (self *cache) copyLocalFolderForConfigure(localFolder string, configureKey string) error {
	destDir := path.Join(self.dir, configureDir, hash(configureKey))

	if err := copy.Copy(localFolder, destDir); err != nil {
		return err
	}

	return nil
}

func (self *cache) aquireLock(key string) error {
	locksDir := path.Join(self.dir, locksDir)
	lockFile := path.Join(locksDir, fmt.Sprintf("%s.lock", hash(key)))

	for {
		if _, err := os.Stat(lockFile); os.IsNotExist(err) {
			break
		}
		slog.Info("Waiting to aquite cache lock")
		time.Sleep(1000 * time.Millisecond)
	}

	if file, err := os.Create(lockFile); os.IsNotExist(err) {
		file.Close()

		if err := os.MkdirAll(locksDir, os.ModePerm); err != nil {
			return errors.Join(errors.New(fmt.Sprintf("Could not create lock file at %s", lockFile)), err)
		}
	}

	return nil
}

func (self *cache) releaseLock(key string) {
	lockFile := path.Join(self.dir, locksDir, fmt.Sprintf("%s.lock", hash(key)))

	if err := os.Remove(lockFile); err != nil {
		slog.Warn(fmt.Sprintf("Failed to remove lock at %s. This may indicate a broader issue.", lockFile))
	}
}
