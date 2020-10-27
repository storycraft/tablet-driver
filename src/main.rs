/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

mod story_tablet;
mod tablet_handler;
mod device;
mod config;
mod tablet;

use std::env;

use story_tablet::StoryTablet;

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