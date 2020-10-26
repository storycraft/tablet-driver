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

use hidapi::HidApi;
use story_tablet::StoryTablet;
use serde_json;

fn main() {
    let device_cfg = serde_json::from_str::<device::Device>(device::DEVICE_CONFIG).expect("Cannot parse device config");
    let config = serde_json::from_str::<config::Config>(config::DEFAULT_CONFIG).expect("Cannot parse config");

    let api = HidApi::new().expect("Cannot create hid handle");

    match StoryTablet::open_new(&api, device_cfg, config) {
        Err(err) => { panic!("Cannot open device: {:?}", err); }

        Ok(res) => {
            let mut tablet = res;

            tablet.start();
        }
    };
}