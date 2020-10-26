/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};
use crate::tablet::Area;

pub const DEVICE_CONFIG: &'static str = include_str!("device.json");

#[derive(Serialize, Deserialize)]
pub struct Device {

    pub name: String,
    
    pub info: Info,
    pub area: Area,

    pub max_pressure: u16,

}

#[derive(Serialize, Deserialize)]
pub struct Info {
    
    pub vendor: u16,
    pub product: u16,
    pub usage: u16,
    pub usage_page: u16,
    pub init_features: Vec<u8>,

}