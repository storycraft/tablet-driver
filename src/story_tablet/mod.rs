/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;
extern crate enigo;

pub mod shared_data;

pub use shared_data::SharedData;

use std::{fs::File, io::Read, sync::Arc, thread::JoinHandle, thread, time::Duration};

use hidapi::HidApi;
use serde_json;
use crate::{device, config, tablet_handler::{TabletHandler, TabletError}};

#[derive(Debug)]
pub enum StoryTabletError {

    NotStarted, AlreadyStarted,

}
pub struct StoryTablet {

    shared: Arc<SharedData>,

    input_handle: Option<JoinHandle<()>>

}

impl StoryTablet {

    pub fn new(device: device::Device, config_arg: Option<String>, auto_restart: bool) -> Self {
        Self {
            shared: Arc::new(SharedData::new(false, auto_restart, device, Self::load_config(config_arg))),

            input_handle: None
        }
    }

    fn load_config(config_arg: Option<String>) -> config::Config {
        if config_arg.is_some() {
            let config_arg = config_arg.unwrap();
    
            let mut file = File::open(&config_arg).expect("Cannot find config file");
            let mut contents = String::new();
    
            if file.metadata().unwrap().len() > 1048576 {
                println!("Config file is too big");
                return Self::load_config(None);
            }
    
            println!("Using {} as config", config_arg);
            file.read_to_string(&mut contents).expect("Cannot read file");
    
            serde_json::from_str::<config::Config>(contents.as_str()).expect("Cannot parse config")
        } else {
            println!("Config not supplied. Proceeding with default");
            serde_json::from_str::<config::Config>(config::DEFAULT_CONFIG).expect("Cannot parse config")
        }
    }
    
    fn create_handle<F>(&self, func: F) -> JoinHandle<()>
    where F: Fn(Arc<SharedData>), F: Send + 'static {
        let handle_shared = Arc::clone(&self.shared);
    
        thread::spawn(move || func(handle_shared))
    }

    pub fn start(mut self) -> Result<(), StoryTabletError> {
        let res = self.shared.start();

        if res.is_err() {
            return res;
        }

        let tablet_handle = self.create_handle(input_task);
        println!("Input thread started. Id: {:?}", tablet_handle.thread().id());

        self.input_handle = Some(tablet_handle);

        println!("Driver started");

        while self.shared.is_started() {
            thread::sleep(Duration::from_secs(16));
        }

        self.do_stop()
    }

    fn do_stop(self) -> Result<(), StoryTabletError> {
        let res = self.shared.stop();
        
        if res.is_err() {
            return res;
        }

        if self.input_handle.is_some() {
            match self.input_handle.unwrap().join() {
                Ok(_) => {
                    println!("Input handler stopped")
                }

                Err(err) => {
                    println!("Error while stopping input thread {:?}", err);
                }
            }
        } else {
            println!("Input handler does not exists?!")
        }
        
        Ok(())
    }

    pub fn stop(&self) -> Result<(), StoryTabletError> {
        self.shared.stop()
    }

    pub fn is_started(&self) -> bool {
        self.shared.is_started()
    }
}

fn input_task(shared_data: Arc<SharedData>) {
    let mut api = HidApi::new().expect("Cannot create hid handle");

    while shared_data.is_started() {
        api.refresh_devices().expect("Cannot update hid device list");

        let auto_restart = shared_data.should_auto_restart();
        
        match TabletHandler::start_new(&api, Arc::clone(&shared_data)) {
            Err(TabletError::NotFound) => {
                if auto_restart {
                    println!("Device not connected. Waiting...");
                } else {
                    println!("Device not connected!!");
                }
                
            }
    
            Err(err) => {
                if auto_restart {
                    println!("Device read error: {:?}", err);
                } else {
                    println!("Error while reading device: {:?}", err);
                }
                
            }
    
            Ok(_) => {
                
            }
        };

        if !auto_restart && !shared_data.is_started() {
            shared_data.stop();
            break;
        }

        thread::sleep(Duration::new(3, 0));
    }
}
