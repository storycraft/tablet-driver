/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};

use crate::{config::Config, device::Device, tablet_handler::TabletStatus};


#[derive(Serialize, Deserialize)]
pub struct ReqCommand {

    pub id: i32,
    pub data: ReqCommands

}

#[derive(Serialize, Deserialize)]
pub struct ResCommand {

    pub id: i32,
    pub data: ResCommands

}

// Client to server
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReqCommands {

    GetConfig {
        
    },
    
    GetStatus {

    },

    GetDevice {

    },

    UpdateConfig {
        config: Config
    },

    SaveConfig {
        force_write: bool
    },

}

// Server to client
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResCommands {

    GetConfig {
        config: Config
    },

    GetStatus {
        status: TabletStatus
    },

    GetDevice {
        device: Device
    },

    UpdateConfig {
        updated: bool
    },

    SaveConfig {
        saved: bool,
        file_changed: bool
    },

}