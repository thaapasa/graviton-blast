use bevy::prelude::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, EnumString, PartialEq, Eq, Clone, Copy, Display)]
pub enum GameImage {
    #[strum(to_string = "player-ship.png")]
    PlayerShip,
}

impl GameImage {
    #[inline]
    fn path(&self) -> String {
        format!("images/{}", self)
    }

    pub fn load(&self, asset_server: Res<AssetServer>) -> Handle<Image> {
        asset_server.load(self.path())
    }
}
