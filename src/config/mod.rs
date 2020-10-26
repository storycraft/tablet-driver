/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use enigo::{Key, MouseButton};
use serde::{Deserialize, Serialize};
use crate::tablet::Area;

pub const DEFAULT_CONFIG: &'static str = include_str!("default.json");

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {

    pub hover_enabled: bool,
    pub buttons: [KeyBinding; 3],

    pub mapping: Area,
    pub screen: Area,

    pub matrix: (f32, f32, f32, f32),

}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "mode")]
pub enum KeyBinding {

    Disabled,
    Mouse { button: MouseButton },
    Keyboard { modifiers: Option<Vec<Key>>, key: Option<char> }

}