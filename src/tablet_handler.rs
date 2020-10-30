/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;

use std::{sync::Arc, time::Duration, thread};

use hidapi::{HidApi, HidError};
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};
use crate::{config::KeyBinding, story_tablet::SharedData};
use crate::tablet::{Data, State};

pub struct TabletHandler {


    shared_data: Arc<SharedData>,

    status: TabletStatus,
    running: bool,

    state: State,
    controller: Enigo,

}

unsafe impl Sync for TabletHandler {

}

pub enum TabletStatus {

    NotConnected,
    Connected,
    Error(HidError)

}

#[derive(Debug)]
pub enum HandlerError {

    AlreadyStarted,
    NotStarted

}

impl TabletHandler {

    pub fn new(shared_data: Arc<SharedData>) -> Self {
        Self {
            shared_data,
            status: TabletStatus::NotConnected,
            running: false,
            state: Default::default(),
            controller: Enigo::new(),
        }
    }

    pub fn start(&mut self) -> Option<HandlerError> {
        if self.running {
            return Some(HandlerError::AlreadyStarted);
        }
        self.running = true;

        println!("Tablet started");

        self.run();

        None
    }

    pub fn stop(&mut self) -> Option<HandlerError> {
        if !self.running {
            return Some(HandlerError::NotStarted);
        }
        self.running = false;

        println!("Stopping");

        None
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn status(&self) -> &TabletStatus {
        &self.status
    }

    fn run(&mut self) {
        let mut hid_api = HidApi::new().expect("Cannot initalize hid device");
        loop {
            let device = self.shared_data.device();
            
            hid_api.refresh_devices().expect("Cannot refresh devices");

            let info = hid_api.device_list().filter(
                |item| 
                item.vendor_id() == device.info.vendor &&
                item.product_id() == device.info.product &&
                item.usage() == device.info.usage &&
                item.usage_page() == device.info.usage_page
            ).next();

            match info {
                None => {
                    println!("Waiting tablet to connect..");
                }

                Some(device_info) => {
                    let hid_device = device_info.open_device(&hid_api).expect("Cannot open device");
            
                    self.status = TabletStatus::Connected;
        
                    println!("Connected to {} {} {}",
                        device_info.manufacturer_string().unwrap_or("Unknown"),
                        device_info.product_string().unwrap_or("Unknown"),
                        device_info.serial_number().unwrap_or("Unknown")
                    );
        
                    let mut buffer = [0_u8; 11];
                    // setup tablet
                    hid_device.send_feature_report(&self.shared_data.device().info.init_features).expect("Cannot init features");
        
                    while self.running {
                        match hid_device.read(&mut buffer) {
                            Err(err) => {
                                println!("Error while reading data {}", err);
        
                                self.status = TabletStatus::Error(err);
                                break;
                            }
            
                            Ok(readed) => {
                                self.on_data(&buffer, readed);
                            }
                        }
                    }
                }
            }
            thread::sleep(Duration::from_secs(1));
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
        if buffer[0] != self.shared_data.device().info.init_features[0] { return; }

        let data = bincode::deserialize::<Data>(buffer).expect("Cannot read data");
        let state = State::from_data(data);

        // println!("{:?}", state);
        
        let config = self.shared_data.get_config().clone();

        if (state.inited || state.hovering) && config.hover_enabled || state.buttons[0] {
            let x = ((state.pos.0 as f32 - config.mapping.x as f32).max(0.0) / config.mapping.width as f32).min(1.0) * config.screen.width as f32;
            let y = ((state.pos.1 as f32 - config.mapping.y as f32).max(0.0) / config.mapping.height as f32).min(1.0) * config.screen.height as f32;

            let win_x = x * config.matrix.0 + y * config.matrix.1;
            let win_y = x * config.matrix.2 + y * config.matrix.3;

            self.controller.mouse_move_to(win_x as i32,win_y as i32);
        }

        for i in 0..3 {
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