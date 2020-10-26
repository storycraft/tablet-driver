/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

pub mod data;
pub mod state;

pub use data::Data;
pub use state::State;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Area {

    pub x: u16,
    pub y: u16,

    pub width: u16,
    pub height: u16

}