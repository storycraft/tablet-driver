/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

mod tablet_handler;
mod device;
mod config;
mod tablet;

extern crate hidapi;
extern crate enigo;

use std::{env, fs::File, io::Read, sync::{Arc, atomic::AtomicBool, Mutex}, thread::JoinHandle, sync::atomic::Ordering, thread, time::Duration};

use hidapi::HidApi;
use serde_json;
use tablet_handler::{TabletHandler, TabletError};

#[derive(Debug)]
pub enum StoryTabletError {

    NotStarted, AlreadyStarted,

}
pub struct StoryTablet {

    started: Arc<AtomicBool>,
    auto_restart: Arc<AtomicBool>,

    device: Arc<device::Device>,
    config: Arc<Mutex<config::Config>>,

    input_handle: Option<JoinHandle<()>>

}

impl StoryTablet {

    pub fn new(device: device::Device, config_arg: Option<String>, auto_restart: bool) -> Self {
        Self {
            started: Arc::new(AtomicBool::new(false)),
            auto_restart: Arc::new(AtomicBool::new(auto_restart)),

            device: Arc::new(device),
            config: Arc::new(Mutex::new(Self::load_config(config_arg))),

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
    where F: Fn(Arc<device::Device>, Arc<Mutex<config::Config>>, Arc<AtomicBool>, Arc<AtomicBool>), F: Send + 'static {
        let handle_device = Arc::clone(&self.device);
        let handle_config = Arc::clone(&self.config);
        let handle_started = Arc::clone(&self.started);
        let handle_auto_restart = Arc::clone(&self.auto_restart);
    
        thread::spawn(move || func(handle_device, handle_config, handle_started, handle_auto_restart))
    }

    pub fn start(mut self) -> Result<(), StoryTabletError> {
        if self.started.load(Ordering::Relaxed) {
            return Err(StoryTabletError::AlreadyStarted);
        }
        self.started.store(true, Ordering::Relaxed);

        let tablet_handle = self.create_handle(input_task);
        println!("Input thread started. Id: {:?}", tablet_handle.thread().id());

        self.input_handle = Some(tablet_handle);

        println!("Driver started");

        while self.started.load(Ordering::Relaxed) {};

        self.stop()
    }

    fn do_stop(self) -> Result<(), StoryTabletError> {
        if !self.started.load(Ordering::Relaxed) {
            return Err(StoryTabletError::NotStarted)
        }
        self.started.store(false, Ordering::Relaxed);

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

    pub fn stop(&self) {
        self.started.store(false, Ordering::Relaxed);
    }

    pub fn started(&self) -> bool {
        self.started.load(Ordering::Relaxed)
    }
}

fn input_task(device: Arc<device::Device>, config: Arc<Mutex<config::Config>>, started: Arc<AtomicBool>, auto_restart: Arc<AtomicBool>) {
    let mut api = HidApi::new().expect("Cannot create hid handle");

    while started.load(Ordering::Relaxed) {
        api.refresh_devices().expect("Cannot update hid device list");

        let auto_restart = auto_restart.load(Ordering::Relaxed);
        
        match TabletHandler::start_new(&api, Arc::clone(&device), Arc::clone(&config), Arc::clone(&started)) {
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

        if !auto_restart {
            started.store(false, Ordering::Relaxed);
            break;
        }

        thread::sleep(Duration::new(3, 0));
    }
}

fn main() {
    let args = env::args();

    let device = serde_json::from_str::<device::Device>(device::DEVICE_CONFIG).expect("Cannot parse device config");
    let config_arg = args.skip(1).next();

    let tablet = StoryTablet::new(device, config_arg, true);

    match tablet.start() {
        Ok(_) => {
            
        }

        Err(err) => {
            panic!("Cannot start driver {:?}", err);
        }
    }
}