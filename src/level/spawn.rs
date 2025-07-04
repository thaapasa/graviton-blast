use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::level::level_data::LevelData;
use crate::{background, black_hole, enemy_ship, player_ship};

impl LevelData {
    pub fn spawn(self, mut commands: Commands, asset_server: Res<AssetServer>) {
        // Spawn background
        for bg in self.background {
            background::spawn_parallax_background(&mut commands, &asset_server, bg);
        }

        // Spawn player
        player_ship::spawn_player_ship(
            &mut commands,
            &asset_server,
            self.player_start.as_location(),
            self.player_start.angle,
        );

        // Spawn black holes
        for spawn_info in self.black_holes {
            black_hole::spawn_black_hole(&mut commands, &asset_server, spawn_info.as_location());
        }

        // Spawn enemies
        for (enemy_type, pos) in self.enemies {
            enemy_ship::spawn_enemy_ship(&mut commands, &asset_server, enemy_type, pos);
        }
    }
}
