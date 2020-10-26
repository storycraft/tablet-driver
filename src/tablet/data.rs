/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {

    pub report_number: u8,

    pub state: u8,

    pub pointer_x: u16,
    pub pointer_y: u16,

    pub pressure: u16,

}

pub mod button_state {

    pub const BUTTON_1: u8 = 0x01;
    pub const BUTTON_2: u8 = 0x02;
    pub const BUTTON_3: u8 = 0x04;

    pub const PEN_DETECTED: u8 = 0x80;
    pub const PEN_INIT: u8 = 0x40;
    pub const PEN_HOVERING: u8 = 0x20;

    pub fn read_state(raw: u8, flag: u8) -> bool {
        raw & flag == flag
    }

}