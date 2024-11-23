package project_config

func (self *ProjectConfig) addFrontend(location string) (*frontendDef, error) {
	frontend := frontendDef{
		location: location,
	}
	self.localConfig.frontends = append(self.localConfig.frontends, frontend)

	return &frontend, nil
}

func (self *ProjectConfig) save() {
	// TODO
}
