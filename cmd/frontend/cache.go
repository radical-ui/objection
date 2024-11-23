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

	"github.com/radical-ui/objection/cmd/project_config"
)

var (
	downloadsDir = "frontend_downloads"
	locksDir     = "locks"
	configureDir = "frontend_configurations"
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

// func (self *cache) getDownloadPath(key string) string {
// 	return path.Join(self.dir, downloadsDir, hash(key))
// }

// func (self *cache) getConfigurePath(key string) string {
// 	return path.Join(self.dir, configureDir, hash(key))
// }

func (self *cache) aquireLock() error {
	// locksDir := path.Join(self.dir, locksDir)
	// lockFile := path.Join(locksDir, fmt.Sprintf("%s.lock", hash(key)))

	for {
		if _, err := os.Stat(self.lockFile); os.IsNotExist(err) {
			break
		}
		slog.Info("Waiting to aquite cache lock")
		time.Sleep(1000 * time.Millisecond)
	}

	if file, err := os.Create(self.lockFile); os.IsNotExist(err) {
		file.Close()

		if err := os.MkdirAll(locksDir, os.ModePerm); err != nil {
			return errors.Join(errors.New(fmt.Sprintf("Could not create lock file at %s", self.lockFile)), err)
		}
	}

	// self.locks = append(self.locks, key)

	return nil
}

func (self *cache) releaseLock() {
	// lockFile := path.Join(self.dir, locksDir, fmt.Sprintf("%s.lock", hash(key)))

	if err := os.Remove(self.lockFile); err != nil {
		slog.Warn(fmt.Sprintf("Failed to remove lock at %s. This may indicate a broader issue.", self.lockFile))
	}

	// newLocks := make([]string, len(self.locks)-1)

	// for _, lock := range self.locks {
	// 	if lock != key {
	// 		newLocks = append(newLocks, lock)
	// 	}
	// }

	// self.locks = newLocks
}

// func (self *cache) releaseAllLocks() {
// 	for _, lock := range self.locks {
// 		self.releaseLock(lock)
// 	}
// }
