use bevy::prelude::*;

use crate::core::Rotation;

#[derive(Default, Resource)]
pub struct PlayerActions {
    pub rotate: Option<Rotation>,
    pub thrust: Option<bool>,
    pub fire: bool,
    pub quit: bool,
}

impl PlayerActions {
    pub fn new() -> Self {
        Self::default()
    }
}
