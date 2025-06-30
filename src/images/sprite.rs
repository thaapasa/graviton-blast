use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;
use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, PartialEq, Eq, Clone, Copy, Display)]
pub enum GameImage {
    #[strum(to_string = "player-ship.png")]
    PlayerShip,
}

impl GameImage {
    /// Returns the rotation (in radians) required to apply to the texture to make
    /// it correspond with the game's natural (mathematical) rotation, where 0 points
    /// to positive X.
    ///
    /// Add this to value to any object's in-game rotation to draw it correctly to the screen.
    pub const INHERENT_TEXTURE_ROTATION: f32 = -FRAC_PI_2;

    #[inline]
    fn path(&self) -> String {
        format!("images/{}", self)
    }

    pub fn load(&self, asset_server: Res<AssetServer>) -> Handle<Image> {
        asset_server.load(self.path())
    }
}
