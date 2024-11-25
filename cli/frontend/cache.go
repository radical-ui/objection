package frontend

import (
	"crypto/md5"
	"encoding/hex"
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path"
	"time"

	"github.com/radical-ui/objection/cli/project_config"
)

type cache struct {
	lockFile     string
	DownloadDir  string
	ConfigureDir string
}

func newCache(info *project_config.FrontendInfo) (*cache, error) {
	dir, ok := os.LookupEnv("OBJECTION_CACHE_DIR")

	if !ok {

		userHome, ok := os.LookupEnv("HOME")
		if !ok {
			return nil, errors.New("Failed to find $HOME env var, as a step to locating a cache dir. Please provide this environment variable, or $OBJECTION_CACHE_DIR, which removes the need to get the home path")
		}

		dir = path.Join(userHome, ".cache/objection")
	}

	var stringKey string
	if len(info.RemoteLocation) != 0 {
		stringKey = fmt.Sprintf("remote:%s", info.RemoteLocation)
	} else {
		stringKey = fmt.Sprintf("local:%s", info.LocalLocation)
	}

	hashKey := md5.Sum([]byte(stringKey))
	key := hex.EncodeToString(hashKey[:])

	return &cache{
		lockFile:     path.Join(dir, "locks", key),
		DownloadDir:  path.Join(dir, "frontend_downloads", key),
		ConfigureDir: path.Join(dir, "frontend_configurations", key),
	}, nil
}

func hash(key string) string {
	hash := md5.Sum([]byte(key))
	return hex.EncodeToString(hash[:])
}

func (self *cache) isDownloaded() bool {
	// we don't just check that the download dir exists, because it could've been created prior to a clone
	// but then the clone failed
	info, err := os.Stat(path.Join(self.DownloadDir, ".git"))

	if os.IsNotExist(err) || !info.IsDir() {
		return false
	}

	return true
}

func (self *cache) aquireLock() error {
	slog.Info("checking to make sure that a lockfile does not already exist", "lockfile", self.lockFile)

	for {
		if _, err := os.Stat(self.lockFile); os.IsNotExist(err) {
			break
		}
		slog.Info("Waiting to aquire cache lock")
		time.Sleep(1000 * time.Millisecond)
	}

	slog.Info("lock file does not exist; creating", "lockfile", self.lockFile)

	if _, err := os.Create(self.lockFile); err != nil {
		slog.Error("failed to create lockfile", "lockfile", self.lockFile, "error", err)

		if os.IsNotExist(err) {
			dir := path.Dir(self.lockFile)

			slog.Info("creating lockfile directory, then creation of lockfile will be retried", "dir", dir)
			if err := os.MkdirAll(dir, os.ModePerm); err != nil {
				return errors.Join(errors.New(fmt.Sprintf("Could not create lock file at %s", self.lockFile)), err)
			}

			if _, err := os.Create(self.lockFile); err != nil {
				slog.Error("failed to create lockfile after creation of directory", "lockfile", self.lockFile, "error", err)

				return errors.Join(fmt.Errorf("failed to create a lock file at '%s'", self.lockFile), err)
			}
		} else {
			return errors.Join(fmt.Errorf("could not create a lock file at '%s'", self.lockFile), err)
		}
	}

	return nil
}

func (self *cache) releaseLock() {
	if err := os.Remove(self.lockFile); err != nil {
		slog.Warn("failed to remove lockfile", "lockfile", self.lockFile, "error", err)
	}
}
