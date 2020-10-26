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
use crate::{device::Device, config::KeyBinding};
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
        self.started = false;
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn get_config(&self) -> Config {
        self.config
    }

    fn run(&mut self) {
        let mut buffer = [0_u8; 11];

        // setup tablet
        self.device.send_feature_report(&self.device_cfg.info.init_features).expect("Cannot init features");

        while self.started {
            let readed = self.device.read(&mut buffer).expect("Couldn't read data");
            self.on_data(&buffer, readed);
        }
    }

    fn down_key(&mut self, binding: KeyBinding) {
        match binding {
            KeyBinding::Mouse { button } => {
                self.controller.mouse_down(button);
            }

            KeyBinding::Keyboard { modifiers, key } => {
                if modifiers.is_some() {
                    self.controller.key_down(modifiers.unwrap());
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
                    self.controller.key_up(modifiers.unwrap());
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

        if (state.inited || state.hovering) && self.config.hover_enabled {
            let x = ((state.pos.0 as f32 - self.config.mapping.x as f32).max(0.0) / self.config.mapping.width as f32).min(1.0) * 1920_f32;
            let y = ((state.pos.1 as f32 - self.config.mapping.y as f32).max(0.0) / self.config.mapping.height as f32).min(1.0) * 1080_f32;

            let win_x = x * self.config.matrix.0 + y * self.config.matrix.1;
            let win_y = x * self.config.matrix.2 + y * self.config.matrix.3;

            self.controller.mouse_move_to(win_x as i32,win_y as i32);
        }

        if state.button1 != self.state.button1 && self.config.buttons.button1.enabled {
            if state.button1 {
                self.down_key(self.config.buttons.button1.binding);
            } else {
                self.up_key(self.config.buttons.button1.binding);
            }
        }

        if state.button2 != self.state.button2 && self.config.buttons.button2.enabled {
            if state.button2 {
                self.down_key(self.config.buttons.button2.binding);
            } else {
                self.up_key(self.config.buttons.button2.binding);
            }
            
        }

        if state.button3 != self.state.button3 && self.config.buttons.button3.enabled {
            if state.button3 {
                self.down_key(self.config.buttons.button3.binding);
            } else {
                self.up_key(self.config.buttons.button3.binding);
            }
        }

        // Update state
        self.state = state;
    }

}