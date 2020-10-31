use crate::{config, device};

/*
 * Created on Wed Oct 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub struct SharedData {

    device: device::Device,
    config: config::Config

}

impl SharedData {

    pub fn new(
        device: device::Device,
        config: config::Config
    ) -> Self {
        Self {
            device,
            config: config
        }
    }

    pub fn device(&self) -> &device::Device {
        &self.device
    }

    pub fn get_config(&self) -> &config::Config {
        &self.config
    }

    pub fn set_config(&mut self, config: config::Config) {
        self.config = config;
    }

}