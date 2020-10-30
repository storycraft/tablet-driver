/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

mod story_tablet;
mod tablet_handler;
mod device;
mod config;
mod command;
mod tablet;

use std::{fs::File, io::Read};

use story_tablet::StoryTablet;

const NAME: &str = "StoryTablet";
const CONFIG_NAME: &str = "last_config.json";

fn main() {
    let device = serde_json::from_str::<device::Device>(device::DEVICE_CONFIG).expect("Cannot parse device config");

    let tablet = StoryTablet::new(NAME, device, load_config(Some(CONFIG_NAME)));

    if tablet.is_err() {
        panic!("Cannot initalize driver {:?}", tablet.err());
    }

    match tablet.unwrap().start() {
        Ok(_) => {
            
        }

        Err(err) => {
            panic!("Cannot start driver {:?}", err);
        }
    }
}

fn load_config(config_path: Option<&str>) -> config::Config {
    if config_path.is_some() {
        let config_path = config_path.unwrap();

        let mut file = File::open(&config_path).expect("Cannot find config file");
        let mut contents = String::new();

        if file.metadata().unwrap().len() > 1048576 {
            println!("Config file is too big");
            return load_config(None);
        }

        println!("Using {} as config", config_path);
        file.read_to_string(&mut contents).expect("Cannot read file");

        serde_json::from_str::<config::Config>(contents.as_str()).expect("Cannot parse config")
    } else {
        println!("Config not supplied. Proceeding with default");
        serde_json::from_str::<config::Config>(config::DEFAULT_CONFIG).expect("Cannot parse config")
    }
}