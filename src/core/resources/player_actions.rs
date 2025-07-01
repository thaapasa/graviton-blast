use crate::core::Rotation;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct PlayerActions {
    pub rotate: Option<Rotation>,
    pub thrust: Option<bool>,
    pub fire: bool,
}

impl PlayerActions {
    pub fn new() -> Self {
        Self::default()
    }
}
