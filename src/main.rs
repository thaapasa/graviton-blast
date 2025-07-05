use bevy::prelude::*;

use crate::background::BackgroundPlugin;
use crate::black_hole::BlackHolePlugin;
use crate::core::resources::PlayerActions;
use crate::core::systems::{
    camera_deadzone_follow, limit_velocity, map_input_to_player_actions, move_all_objects,
    rotate_all_objects, rotate_to_match_velocity,
};
use crate::enemy_ship::EnemyShipPlugin;
use crate::level::Level1;
use crate::player_ship::PlayerShipPlugin;
use crate::projectile::ProjectilePlugin;

mod assets;
mod background;
mod black_hole;
mod constants;
mod core;
mod enemy_ship;
mod level;
mod player_ship;
mod projectile;
mod utils;

#[cfg(test)]
pub mod tests;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            PlayerShipPlugin,
            EnemyShipPlugin,
            BackgroundPlugin,
            BlackHolePlugin,
            ProjectilePlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(PlayerActions::new())
        .add_systems(Update, map_input_to_player_actions)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_to_match_velocity)
        .add_systems(
            Update,
            (limit_velocity, move_all_objects, rotate_all_objects),
        )
        .add_systems(Update, camera_deadzone_follow)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);
    // Start with level 1
    Level1::create().spawn(commands, asset_server);
}
