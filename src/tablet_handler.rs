/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;

use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

use hidapi::HidApi;
use hidapi::HidDevice;
use hidapi::DeviceInfo;
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};
use crate::{config::KeyBinding, device::Device};
use crate::tablet::{Data, State};
use crate::config::Config;

pub struct TabletHandler {

    device_info: DeviceInfo,
    device: Arc<Device>,

    hid_device: HidDevice,

    config: Arc<Mutex<Config>>,

    state: State,
    controller: Enigo,

    started: Arc<AtomicBool>

}

#[derive(Debug)]
pub enum TabletError {

    NotFound,
    InvalidAccess

}

impl TabletHandler {

    pub fn start_new(hid_api: &HidApi, device: Arc<Device>, config: Arc<Mutex<Config>>, started: Arc<AtomicBool>) -> Result<Self, TabletError> {

        let hid_device = hid_api.device_list().filter(
            |item| 
            item.vendor_id() == device.info.vendor &&
            item.product_id() == device.info.product &&
            item.usage() == device.info.usage &&
            item.usage_page() == device.info.usage_page
        ).next();

        match hid_device {
            None => { Err(TabletError::NotFound) },

            Some(device_info) => {
                match device_info.open_device(hid_api) {
                    Err(_) => { Err(TabletError::InvalidAccess) }

                    Ok(hid_device) => {
                        let mut handler = Self {
                            device_info: device_info.clone(),

                            hid_device,
                            device,

                            config,

                            state: Default::default(),
                            controller: Enigo::new(),

                            started
                        };

                        handler.start();
                        Ok(handler)
                    }
                }
            }
        }
        
        
    }

    fn start(&mut self) {
        println!("Tablet started");
        println!("Connected to {} {} {}",
            self.device_info.manufacturer_string().unwrap_or("Unknown"),
            self.device_info.product_string().unwrap_or("Unknown"),
            self.device_info.serial_number().unwrap_or("Unknown")
        );

        self.run()
    }

    fn stop(&mut self) {
        println!("Stopping");
    }

    fn run(&mut self) {
        let mut buffer = [0_u8; 11];
        // setup tablet
        self.hid_device.send_feature_report(&self.device.info.init_features).expect("Cannot init features");

        while self.started.load(Ordering::Relaxed) {
            match self.hid_device.read(&mut buffer) {
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

            KeyBinding::Disabled => {

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

            KeyBinding::Disabled => {

            }
        }
    }

    fn on_data(&mut self, buffer: &[u8; 11], _: usize) {
        if buffer[0] != self.device.info.init_features[0] { return; }

        let data = bincode::deserialize::<Data>(buffer).expect("Cannot read data");
        let state = State::from_data(data);

        //println!("{:?}", state);
        
        let config = self.config.lock().unwrap().clone();

        if (state.inited || state.hovering) && config.hover_enabled || state.buttons[0] {
            let x = ((state.pos.0 as f32 - config.mapping.x as f32).max(0.0) / config.mapping.width as f32).min(1.0) * config.screen.width as f32;
            let y = ((state.pos.1 as f32 - config.mapping.y as f32).max(0.0) / config.mapping.height as f32).min(1.0) * config.screen.height as f32;

            let win_x = x * config.matrix.0 + y * config.matrix.1;
            let win_y = x * config.matrix.2 + y * config.matrix.3;

            self.controller.mouse_move_to(win_x as i32,win_y as i32);
        }

        for i in 0..2 {
            if state.buttons[i] != self.state.buttons[i] {
                let binding = config.buttons[i].clone();
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