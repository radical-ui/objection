package frontend

import (
	"github.com/otiai10/copy"
	"os"
	"path"

	"github.com/radical-ui/objection/cmd/project_config"
)

type FrontendManager struct {
	info  *project_config.FrontendInfo
	cache *cache
	git   *git
}

func NewFrontendManager(info *project_config.FrontendInfo) (*FrontendManager, error) {
	cache, err := newCache(info)
	if err != nil {
		return nil, err
	}

	git := newGit(cache, info.RemoteLocation)

	return &FrontendManager{info, cache, git}, nil
}

func (self *FrontendManager) Lock() error {
	return self.cache.aquireLock()
}

func (self *FrontendManager) ReleaseLock() {
	self.cache.releaseLock()
}

func (self *FrontendManager) ShouldDownload() bool {
	return len(self.info.RemoteLocation) != 0 && !self.cache.isDownloaded()
}

func (self *FrontendManager) Download() error {
	if err := self.git.downloadRepo(); err != nil {
		return err
	}

	return nil
}

type RevInfo struct {
	DefaultRev string
	Tags       []string
	Commits    []Commit
}

func (self *FrontendManager) GetRevInfo() (RevInfo, error) {
	tags, err := self.git.listAllTags()
	if err != nil {
		return RevInfo{}, err
	}

	commits, err := self.git.getRecentCommits(20)
	if err != nil {
		return RevInfo{}, err
	}

	defaultRev, err := self.git.getDefaultRev()
	if err != nil {
		return RevInfo{}, err
	}

	return RevInfo{
		defaultRev,
		tags,
		commits,
	}, nil
}

func (self *FrontendManager) EnsureRevIsApplied() error {
	if len(self.info.RemoteLocation) != 0 {
		return self.git.checkoutRev(self.info.RemoteRev)
	}

	return nil
}

func (self *FrontendManager) GetConfigPath() string {
	configFile := "objection_frontend.hcl"

	if len(self.info.RemoteLocation) != 0 {
		return path.Join(self.cache.DownloadDir, configFile)
	}

	return path.Join(self.info.LocalLocation, configFile)
}

func (self *FrontendManager) PrepareToConfigure() (string, error) {
	if len(self.info.RemoteLocation) != 0 {
		if err := gitFriendlyCopy(self.cache.DownloadDir, self.cache.ConfigureDir); err != nil {
			return "", err
		}
	} else {
		if err := gitFriendlyCopy(self.info.LocalLocation, self.cache.ConfigureDir); err != nil {
			return "", err
		}
	}

	return self.cache.ConfigureDir, nil
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
