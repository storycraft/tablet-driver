/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;
extern crate enigo;

pub mod shared_data;

use raw_sync::events::{Event, EventInit, EventState};
pub use shared_data::SharedData;
use shared_memory::{Shmem, ShmemConf, ShmemError};

use std::{sync::Arc, thread::JoinHandle, sync::Mutex, thread, time::Duration};

use crate::{config::Config, device, tablet_handler::TabletHandler};

#[derive(Debug)]
pub enum StoryTabletError {

    NotStarted, AlreadyStarted,
    InstanceConflict(ShmemError)

}
pub struct StoryTablet {

    name: &'static str,

    shared_mem: Shmem,

    started: bool,
    shared: Arc<SharedData>,

    tablet_handler: Arc<Mutex<TabletHandler>>

}

impl StoryTablet {

    pub fn new(name: &'static str,device: device::Device, config: Config) -> Result<Self, StoryTabletError> {
        let shared_data = Arc::new(SharedData::new(device, config));
        let shared_mem = ShmemConf::new().size(4096).flink(name).create();
        if shared_mem.is_err() {
            return Err(StoryTabletError::InstanceConflict(shared_mem.err().unwrap()))
        }

        Ok(Self {
            name,
            shared_mem: shared_mem.unwrap(),
            started: false,
            shared: Arc::clone(&shared_data),

            tablet_handler: Arc::new(Mutex::new(TabletHandler::new(Arc::clone(&shared_data))))
        })
    }
    
    fn create_handle<F>(&self, func: F) -> JoinHandle<()>
    where F: Fn(), F: Send + 'static {
        thread::spawn(move || func())
    }

    pub fn start(mut self) -> Result<(), StoryTabletError> {
        if self.started {
            return Err(StoryTabletError::AlreadyStarted);
        }
        self.started = true;

        let inner_handler = Arc::clone(&self.tablet_handler);
        let input_handle = self.create_handle(move || {
            inner_handler.lock().unwrap().start();
        });
        println!("Input thread started. Id: {:?}", input_handle.thread().id());

        println!("Driver started");

        while self.started {
            let (e, used_bytes) = unsafe { Event::from_existing(self.shared_mem.as_ptr()).unwrap() };
            thread::sleep(Duration::from_millis(1));

            e.set(EventState::Signaled).expect("Cannot signal event");
        }

        let mut tablet_handler = self.tablet_handler.lock().unwrap();
        if tablet_handler.running() {
            tablet_handler.stop();
        }
        input_handle.join().expect("Input thread already killed");

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), StoryTabletError> {
        if !self.started {
            return Err(StoryTabletError::NotStarted);
        }
        self.started = false;
        
        Ok(())
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
