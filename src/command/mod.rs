/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Deserialize, Serialize};

use crate::config::Config;

// Client to server
#[derive(Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum ReqCommand {

    Stop {

    },

    GetConfig {
        
    },

    UpdateConfig {
        config: Config
    },

}

// Server to client
#[derive(Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum ResCommand {

    Stop {
        stopping: bool
    },

    GetConfig {
        config: Config
    },

    UpdateConfig {
        updated: bool
    },

}