/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use enigo::{Key, MouseButton};
use serde::{Deserialize, Serialize};
use crate::tablet::Area;

pub const DEFAULT_CONFIG: &'static str = include_str!("default.json");

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Config {

    pub hover_enabled: bool,
    pub buttons: ButtonMap,
    pub mapping: Area,

    pub matrix: (f32, f32, f32, f32),

}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct ButtonMap {

    pub button1: ButtonData,
    pub button2: ButtonData,
    pub button3: ButtonData,

}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct ButtonData {

    pub enabled: bool,
    pub binding: KeyBinding,

}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "type")]
pub enum KeyBinding {

    Mouse { button: MouseButton },
    Keyboard { modifiers: Option<Key>, key: Option<char> }

}