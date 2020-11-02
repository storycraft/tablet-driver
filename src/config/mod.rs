/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use std::{fs::File, fs::{self, OpenOptions}, io, io::Read, path::PathBuf, io::Write};

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

impl Config {

    pub fn load_from_file(file: &mut File) -> Result<Self, ConfigError> {
        if file.metadata().unwrap().len() > 1048576 {
            return Err(ConfigError::TooLarge);
        }

        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(err) => {
                Err(ConfigError::Io(err))
            }

            Ok(_) => {
                Self::load_from_content(content.as_str())
            }
        }
    }

    pub fn load_from_content(content: &str) -> Result<Self, ConfigError> {
        let config_res = serde_json::from_str::<Config>(content);

        if config_res.is_err() {
            return Err(ConfigError::JsonParse(config_res.err().unwrap()));
        }

        Ok(config_res.unwrap())
    }

    pub fn save_to_file(&self, file: &mut File, pretty: bool) -> Result<(), ConfigError> {
        let content = if pretty { serde_json::to_string_pretty(self) } else { serde_json::to_string(self) };
        if content.is_err() {
            return Err(ConfigError::JsonParse(content.err().unwrap()));
        }
        
        let written = file.write(content.unwrap().as_bytes());
        if written.is_err() {
            return Err(ConfigError::Io(written.err().unwrap()));
        }

        Ok(())
    }

}

pub struct ConfigFile {

    file: File,
    config: Config

}

#[derive(Debug)]
pub enum ConfigError {
    TooLarge,
    JsonParse(serde_json::Error),
    Io(io::Error)
}

impl ConfigFile {

    pub fn new(file: File, config: Config) -> Self {
        Self {
            file,
            config
        }
    }

    pub fn from_path(path: &String) -> Result<Self, ConfigError> {
        match OpenOptions::new().write(true).read(true).open(path) {
            Ok(mut file) => {

                let config_res = Config::load_from_file(&mut file);

                if config_res.is_err() {
                    return Err(config_res.err().unwrap());
                }

                Ok(Self {
                    file,
                    config: config_res.unwrap()
                })
            }

            Err(err) => {
                Err(ConfigError::Io(err))
            }
        }
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn reload_from_file(&mut self) -> Result<(), ConfigError> {
        let config = Config::load_from_file(&mut self.file);

        if config.is_err() {
            return Err(config.err().unwrap());
        }

        self.set_config(config.unwrap());
        Ok(())
    }

    pub fn save_to_file(&mut self, pretty: bool) -> Result<(), ConfigError> {
        self.config.save_to_file(&mut self.file, pretty)
    }

}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "mode")]
pub enum KeyBinding {

    Disabled,
    Mouse { button: MouseButton },
    Keyboard { modifiers: Option<Vec<Key>>, key: Option<char> }

}