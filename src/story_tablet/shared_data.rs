use crate::{config::{Config, ConfigFile}, device::Device};

/*
 * Created on Wed Oct 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub struct SharedData {

    device: Device,
    config_file: ConfigFile

}

impl SharedData {

    pub fn new(
        device: Device,
        config_file: ConfigFile
    ) -> Self {
        Self {
            device,
            config_file
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn get_config_file(&self) -> &ConfigFile {
        &self.config_file
    }

    pub fn get_config_file_mut(&mut self) -> &mut ConfigFile {
        &mut self.config_file
    }

    pub fn config(&self) -> &Config {
        self.config_file.get_config()
    }

    pub fn set_config_file(&mut self, config: ConfigFile) {
        self.config_file = config;
    }

}