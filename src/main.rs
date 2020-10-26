/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

mod story_tablet;
mod device;
mod config;
mod tablet;

extern crate hidapi;
extern crate enigo;

use std::{time::Duration, env, fs::File, io::Read, thread};

use hidapi::HidApi;
use serde_json;
use story_tablet::{StoryTablet, TabletError};

fn main() {
    let args = env::args();
    let config_arg = args.skip(1).next();

    let config = load_config(config_arg);

    let device_cfg = serde_json::from_str::<device::Device>(device::DEVICE_CONFIG).expect("Cannot parse device config");

    loop {
        let api = HidApi::new().expect("Cannot create hid handle");
        match StoryTablet::open_new(&api, device_cfg.clone(), config.clone()) {
            Err(TabletError::NotFound) => {
                println!("Device not connected. Waiting...");
            }
    
            Err(err) => {
                panic!("Cannot open device: {:?}", err);
            }
    
            Ok(res) => {
                let mut tablet = res;
    
                tablet.start();
            }
        };

        thread::sleep(Duration::new(5, 0));
    }
}

fn load_config(config_arg: Option<String>) -> config::Config {
    if config_arg.is_some() {
        let config_arg = config_arg.unwrap();

        let mut file = File::open(&config_arg).expect("Cannot find config file");
        let mut contents = String::new();

        if file.metadata().unwrap().len() > 1048576 {
            println!("Config file is too big");
            return load_config(None);
        }

        println!("Using {} as config", config_arg);
        file.read_to_string(&mut contents).expect("Cannot read file");

        serde_json::from_str::<config::Config>(contents.as_str()).expect("Cannot parse config")
    } else {
        println!("Config not supplied. Proceeding with default");
        serde_json::from_str::<config::Config>(config::DEFAULT_CONFIG).expect("Cannot parse config")
    }
}