use std::sync::{Mutex, MutexGuard};

use crate::{config, device};

/*
 * Created on Wed Oct 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub struct SharedData {

    device: device::Device,
    config: Mutex<config::Config>

}

impl SharedData {

    pub fn new(
        device: device::Device,
        config: config::Config
    ) -> Self {
        Self {
            device,
            config: Mutex::new(config)
        }
    }

    pub fn device(&self) -> &device::Device {
        &self.device
    }

    pub fn get_config(&self) -> MutexGuard<config::Config> {
        self.config.lock().unwrap()
    }

    pub fn set_config(&self, config: config::Config) {
        self.config.lock().unwrap().clone_from(&config);
    }

}