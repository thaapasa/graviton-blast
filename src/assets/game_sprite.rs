use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameSprite {
    PlayerShip,
}

impl GameSprite {
    /// Returns the rotation (in radians) required to apply to the texture to make
    /// it correspond with the game's natural (mathematical) rotation, where 0 points
    /// to positive X.
    ///
    /// Add this to value to any object's in-game rotation to draw it correctly to the screen.
    pub const INHERENT_TEXTURE_ROTATION: f32 = -FRAC_PI_2;

    #[inline]
    pub fn filename(&self) -> &str {
        match self {
            Self::PlayerShip => "player-ship",
        }
    }

    #[inline]
    pub fn scale(&self) -> f32 {
        match self {
            Self::PlayerShip => 0.2,
        }
    }

    #[inline]
    fn path(&self) -> String {
        format!("sprites/{}.png", self.filename())
    }

    pub fn load(&self, asset_server: Res<AssetServer>) -> Handle<Image> {
        asset_server.load(self.path())
    }
}
