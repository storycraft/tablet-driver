use std::sync::{Mutex, MutexGuard, atomic::{AtomicBool, Ordering}};

use crate::{config, device};

use super::StoryTabletError;

/*
 * Created on Wed Oct 28 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub struct SharedData {

    started: AtomicBool,
    auto_restart: AtomicBool,

    device: device::Device,
    config: Mutex<config::Config>

}

impl SharedData {

    pub fn new(
        started: bool,
        auto_restart: bool,
        device: device::Device,
        config: config::Config
    ) -> Self {
        Self {
            started: AtomicBool::new(started),
            auto_restart: AtomicBool::new(auto_restart),
            device,
            config: Mutex::new(config)
        }
    }

    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::Relaxed)
    }

    pub fn start(&self) -> Result<(), StoryTabletError>{
        if !self.started.compare_and_swap(false, true, Ordering::Relaxed) {
            return Err(StoryTabletError::AlreadyStarted);
        }

        Ok(())
    }

    pub fn stop(&self) -> Result<(), StoryTabletError>{
        if !self.started.compare_and_swap(true, false, Ordering::Relaxed) {
            return Err(StoryTabletError::NotStarted);
        }

        Ok(())
    }

    pub fn should_auto_restart(&self) -> bool {
        self.auto_restart.load(Ordering::Relaxed)
    }

    pub fn set_auto_restart(&self, val: bool) {
        self.auto_restart.store(val, Ordering::Relaxed);
    }

    pub fn device(&self) -> &device::Device {
        &self.device
    }

    pub fn get_config(&self) -> MutexGuard<config::Config> {
        self.config.lock().unwrap()
    }

    pub fn set_config(&mut self, config: config::Config) {
        self.config = Mutex::new(config);
    }

}