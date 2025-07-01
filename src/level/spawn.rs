use crate::level::level_data::LevelData;
use crate::player_ship;
use bevy::asset::AssetServer;
use bevy::prelude::*;

impl LevelData {
    pub fn spawn(&self, mut commands: Commands, asset_server: Res<AssetServer>) {
        // Spawn player
        player_ship::spawn::spawn_player_ship(
            &mut commands,
            asset_server,
            self.player_start.as_location(),
            self.player_start.angle,
        );

        // Spawn black holes
        for _ in &self.black_holes {
            // TODO: Spawn
        }
    }
}
