package frontend

import (
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

type SetupParams struct {
	// If specified
	SelectRev func(defaultTag string, otherTagsOrHashes []string) string
}

func (self *FrontendManager) DownloadIfNecessary() error {
	// remoteDownloadKey := fmt.Sprintf("download:%s", self.info.RemoteLocation)
	// defer self.cache.releaseAllLocks()

	if len(self.info.RemoteLocation) != 0 {
		// self.cache.aquireLock(remoteDownloadKey)
		if err := self.git.downloadRepo(); err != nil {
			return err
		}

		// if len(self.info.RemoteRev) == 0 {
		// 	tags, err := self.git.listAllTags(self.info.RemoteLocation)
		// 	if err != nil {
		// 		return err
		// 	}

		// 	self.info.RemoteRev = params.SelectRev(tags)
		// }

		// self.git.checkoutTag(self.info.RemoteLocation, self.info.RemoteRev)
	}

	return nil
}

type RevInfo struct {
	defaultRev   []string
	currentHash  string
	tagsOrHashes []string
}

func (self *FrontendManager) GetRevInfo() (RevInfo, error) {
	// self.git.listAllTags()
	// TODO

	return RevInfo{}, nil
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
		if err := self.git.copyDownloadedRepoForConfiguration(self.info.RemoteRev); err != nil {
			return "", err
		}
	} else {
		if err := self.git.copyLocalFolderForConfiguration(self.info.LocalLocation); err != nil {
			return "", err
		}
	}

	return self.cache.ConfigureDir, nil
}
