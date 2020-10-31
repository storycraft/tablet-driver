/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;

use serde::{Deserialize, Serialize};
use std::{sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}}, thread, time::Duration};

use hidapi::{HidApi, HidDevice};
use enigo::{Enigo, Key, KeyboardControllable, MouseControllable};
use crate::{config::KeyBinding, story_tablet::SharedData};
use crate::tablet::{Data, State};

pub struct TabletHandler {

    shared_data: Arc<RwLock<SharedData>>,

    status: RwLock<TabletStatus>,
    running: AtomicBool,

    state: RwLock<State>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TabletStatus {

    NotConnected,
    Connected,
    Error

}

#[derive(Debug)]
pub enum HandlerError {

    AlreadyStarted,
    NotStarted

}

impl TabletHandler {

    pub fn new(shared_data: Arc<RwLock<SharedData>>) -> Self {
        Self {
            shared_data,
            status: RwLock::new(TabletStatus::NotConnected),
            running: AtomicBool::new(false),
            state: Default::default(),
        }
    }

    pub fn start(&self) -> Option<HandlerError> {
        if self.running.load(Ordering::Relaxed) {
            return Some(HandlerError::AlreadyStarted);
        }
        self.running.store(true, Ordering::Relaxed);

        println!("Tablet started");

        self.run();

        None
    }

    pub fn stop(&self) -> Option<HandlerError> {
        if !self.running.load(Ordering::Relaxed) {
            return Some(HandlerError::NotStarted);
        }
        self.running.store(false, Ordering::Relaxed);

        println!("Stopping");

        None
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn get_status(&self) -> TabletStatus {
        self.status.read().unwrap().clone()
    }

    pub fn set_status(&self, status: TabletStatus) {
        *self.status.write().unwrap() = status;
    }

    fn run(&self) {
        let mut hid_api = HidApi::new().expect("Cannot initalize hid device");
        let mut controller = Enigo::new();
        
        let mut buffer = [0_u8; 11];
        let mut hid_device: Option<HidDevice> = None;

        while self.running.load(Ordering::Relaxed) {
            match self.get_status() {
                TabletStatus::Connected => {
                    if hid_device.is_none() {
                        self.set_status(TabletStatus::NotConnected);
                        continue;
                    }

                    while self.running.load(Ordering::Relaxed) {
                        match hid_device.as_ref().unwrap().read(&mut buffer) {
                            Err(err) => {
                                println!("Error while reading data {}", err);
        
                                self.set_status(TabletStatus::Error);
                                break;
                            }
            
                            Ok(readed) => {
                                self.on_data(&mut controller, &buffer, readed);
                            }
                        }
                    }
                }

                TabletStatus::NotConnected | TabletStatus::Error => {
                    let shared_data = self.shared_data.read().unwrap();

                    let device = shared_data.device();
                    
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
                            let hid = device_info.open_device(&hid_api).expect("Cannot open device");
                    
                            self.set_status(TabletStatus::Connected);
                
                            println!("Connected to {} {} {}",
                                device_info.manufacturer_string().unwrap_or("Unknown"),
                                device_info.product_string().unwrap_or("Unknown"),
                                device_info.serial_number().unwrap_or("Unknown")
                            );
                            // setup tablet
                            hid.send_feature_report(&shared_data.device().info.init_features).expect("Cannot init features");

                            hid_device = Some(hid);
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    }

    fn down_key(&self, controller: &mut Enigo, binding: KeyBinding) {
        match binding {
            KeyBinding::Mouse { button } => {
                controller.mouse_down(button);
            }

            KeyBinding::Keyboard { modifiers, key } => {
                if modifiers.is_some() {
                    modifiers.clone().unwrap().iter().for_each(|modifer_key| controller.key_down(*modifer_key));
                }
                
                if key.is_some() {
                    controller.key_down(Key::Layout(key.unwrap()));
                }
            }

            KeyBinding::Disabled => {

            }
        }
    }

    fn up_key(&self, controller: &mut Enigo, binding: KeyBinding) {
        match binding {
            KeyBinding::Mouse { button } => {
                controller.mouse_up(button);
            }

            KeyBinding::Keyboard { modifiers, key } => {
                if modifiers.is_some() {
                    modifiers.clone().unwrap().iter().for_each(|modifer_key| controller.key_up(*modifer_key));
                }
                
                if key.is_some() {
                    controller.key_up(Key::Layout(key.unwrap()));
                }
            }

            KeyBinding::Disabled => {

            }
        }
    }

    fn on_data(&self, controller: &mut Enigo, buffer: &[u8; 11], _: usize) {
        if buffer[0] != 2 { return; }

        let data = bincode::deserialize::<Data>(buffer).expect("Cannot read data");
        let state = State::from_data(data);
        let mut prev_state = self.state.write().unwrap();

        // println!("{:?}", state);
        
        let config = self.shared_data.read().unwrap().get_config().clone();

        if (state.inited || state.hovering) && config.hover_enabled || state.buttons[0] {
            let x = ((state.pos.0 as f32 - config.mapping.x as f32).max(0.0) / config.mapping.width as f32).min(1.0) * config.screen.width as f32;
            let y = ((state.pos.1 as f32 - config.mapping.y as f32).max(0.0) / config.mapping.height as f32).min(1.0) * config.screen.height as f32;

            let win_x = x * config.matrix.0 + y * config.matrix.1;
            let win_y = x * config.matrix.2 + y * config.matrix.3;

            controller.mouse_move_to(win_x as i32,win_y as i32);
        }

        for i in 0..3 {
            if state.buttons[i] != prev_state.buttons[i] {
                let binding = config.buttons[i].clone();
                if state.buttons[i] {
                    self.down_key(controller, binding);
                } else {
                    self.up_key(controller, binding);
                }
            }
        }

        // Update state
        *prev_state = state;
    }

}