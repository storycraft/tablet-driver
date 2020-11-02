/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod story_tablet;
mod tablet_handler;
mod device;
mod config;
mod command;
mod tablet;

use std::{env, fs::{self, OpenOptions}};

use config::ConfigFile;
use story_tablet::StoryTablet;

const DEFAULT_CONFIG: &str = "config.json";
const PORT: u16 = 55472;

fn main() {
    let device = serde_json::from_str::<device::Device>(device::DEVICE_CONFIG).expect("Cannot parse device config");

    let config_path = env::args().nth(1).unwrap_or(String::from("config.json"));

    let default_config = config::Config::load_from_content(config::DEFAULT_CONFIG).expect("Cannot load default config. This should not happen");

    println!("Using {} as config", config_path.as_str());
    let mut config_file = match ConfigFile::from_path(&config_path) {
        Err(err) => {
            println!("Error while reading config {:?}. Proceeding with default", err);
            ConfigFile::new(
                OpenOptions::new().read(true).write(true).create(true).open(DEFAULT_CONFIG).expect("Cannot create file for default config"),
                default_config
            )
        }

        Ok(loaded_config_file) => {
            loaded_config_file
        }
    };
    let write_res = config_file.save_to_file(true);
    if write_res.is_err() {
        println!("Cannot save config. {:?}", write_res.err().unwrap());
    }

    let tablet = StoryTablet::new(PORT, device, config_file);

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