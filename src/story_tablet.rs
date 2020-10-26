/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;

use hidapi::HidApi;
use hidapi::HidDevice;
use hidapi::DeviceInfo;
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};
use crate::{config::KeyBinding, device::Device};
use crate::tablet::{Data, State};
use crate::config::Config;

pub struct StoryTablet {

    device_info: DeviceInfo,
    device_cfg: Device,

    device: HidDevice,

    config: Config,

    state: State,
    controller: Enigo,

    started: bool

}

#[derive(Debug)]
pub enum TabletError {

    NotFound,
    InvalidAccess

}

impl StoryTablet {

    pub fn open_new(hid_api: &HidApi, device_cfg: Device, config: Config) -> Result<Self, TabletError> {

        let device = hid_api.device_list().filter(
            |item| 
            item.vendor_id() == device_cfg.info.vendor &&
            item.product_id() == device_cfg.info.product &&
            item.usage() == device_cfg.info.usage &&
            item.usage_page() == device_cfg.info.usage_page
        ).next();

        match device {
            None => { Err(TabletError::NotFound) },

            Some(device_info) => {
                match device_info.open_device(hid_api) {
                    Err(_) => { Err(TabletError::InvalidAccess) }

                    Ok(device) => {
                        Ok(Self {
                            device_info: device_info.clone(),

                            device,
                            device_cfg,

                            config,

                            state: Default::default(),
                            controller: Enigo::new(),

                            started: false
                        })
                    }
                }
            }
        }
        
        
    }

    pub fn start(&mut self) {
        if self.started {
            println!("Driver already started");
            return;
        }
        self.started = true;

        println!("Driver started");
        println!("Connected to {} {} {}",
            self.device_info.manufacturer_string().unwrap_or("Unknown"),
            self.device_info.product_string().unwrap_or("Unknown"),
            self.device_info.serial_number().unwrap_or("Unknown")
        );

        self.run()
    }

    pub fn stop(&mut self) {
        if !self.started {
            println!("Driver not started");
            return;
        }

        println!("Stopping");
        self.started = false;
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn run(&mut self) {
        let mut buffer = [0_u8; 11];

        // setup tablet
        self.device.send_feature_report(&self.device_cfg.info.init_features).expect("Cannot init features");

        while self.started {
            match self.device.read(&mut buffer) {
                Err(err) => {
                    self.stop();
                    println!("Error while reading data {}", err);
                }

                Ok(readed) => {
                    self.on_data(&buffer, readed);
                }
            }
        }
    }

    fn down_key(&mut self, binding: KeyBinding) {
        match binding {
            KeyBinding::Mouse { button } => {
                self.controller.mouse_down(button);
            }

            KeyBinding::Keyboard { modifiers, key } => {
                if modifiers.is_some() {
                    modifiers.clone().unwrap().iter().for_each(|modifer_key| self.controller.key_down(*modifer_key));
                }
                
                if key.is_some() {
                    self.controller.key_down(Key::Layout(key.unwrap()));
                }
            }
        }
    }

    fn up_key(&mut self, binding: KeyBinding) {
        match binding {
            KeyBinding::Mouse { button } => {
                self.controller.mouse_up(button);
            }

            KeyBinding::Keyboard { modifiers, key } => {
                if modifiers.is_some() {
                    modifiers.clone().unwrap().iter().for_each(|modifer_key| self.controller.key_up(*modifer_key));
                }
                
                if key.is_some() {
                    self.controller.key_up(Key::Layout(key.unwrap()));
                }
            }
        }
    }

    fn on_data(&mut self, buffer: &[u8; 11], _: usize) {
        if buffer[0] != self.device_cfg.info.init_features[0] { return; }

        let data = bincode::deserialize::<Data>(buffer).expect("Cannot read data");
        let state = State::from_data(data);

        //println!("{:?}", state);

        if (state.inited || state.hovering) && self.config.hover_enabled || state.buttons[0] {
            let x = ((state.pos.0 as f32 - self.config.mapping.x as f32).max(0.0) / self.config.mapping.width as f32).min(1.0) * self.config.screen.width as f32;
            let y = ((state.pos.1 as f32 - self.config.mapping.y as f32).max(0.0) / self.config.mapping.height as f32).min(1.0) * self.config.screen.height as f32;

            let win_x = x * self.config.matrix.0 + y * self.config.matrix.1;
            let win_y = x * self.config.matrix.2 + y * self.config.matrix.3;

            self.controller.mouse_move_to(win_x as i32,win_y as i32);
        }

        for i in 0..2 {
            if state.buttons[i] != self.state.buttons[i] && self.config.buttons[i].enabled {
                let binding = self.config.buttons[i].binding.clone();
                if state.buttons[i] {
                    self.down_key(binding);
                } else {
                    self.up_key(binding);
                }
            }
        }

        // Update state
        self.state = state;
    }

}