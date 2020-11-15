/*
 * Created on Mon Oct 26 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use super::{Data, data::button_state};
#[derive(Default, Debug)]
pub struct State {

    pub pos: (u16, u16),

    pub hovering: bool,
    pub inited: bool,
    pub detected: bool,

    pub buttons: [bool; 3]

}

impl State {

    pub fn from_data(data: Data) -> Self {
        Self {
            pos: (data.pointer_x, data.pointer_y),

            hovering: button_state::read_state(data.state, button_state::PEN_HOVERING),
            inited: button_state::read_state(data.state, button_state::PEN_INIT),
            detected: button_state::read_state(data.state, button_state::PEN_DETECTED),
    
            buttons: [
                button_state::read_state(data.state, button_state::BUTTON_1),
                button_state::read_state(data.state, button_state::BUTTON_2),
                button_state::read_state(data.state, button_state::BUTTON_3)
            ]
        }
    }

}