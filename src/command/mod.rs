/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};

use crate::{config::Config, tablet_handler::TabletStatus};

// Client to server
#[derive(Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum ReqCommand {

    GetConfig {
        
    },
    
    GetStatus {

    },

    UpdateConfig {
        config: Config
    },

}

// Server to client
#[derive(Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum ResCommand {

    GetConfig {
        config: Config
    },

    GetStatus {
        status: TabletStatus
    },

    UpdateConfig {
        updated: bool
    },

}