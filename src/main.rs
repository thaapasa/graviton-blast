use crate::background::BackgroundPlugin;
use crate::black_hole::BlackHolePlugin;
use crate::core::resources::PlayerActions;
use crate::core::systems::{
    camera_deadzone_follow, map_input_to_player_actions, move_all_objects, rotate_all_objects,
};
use crate::level::Level1;
use crate::player_ship::PlayerShipPlugin;
use bevy::prelude::*;

mod assets;
mod background;
mod core;
mod level;
mod player_ship;

mod black_hole;
#[cfg(test)]
pub mod tests;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerShipPlugin, BackgroundPlugin, BlackHolePlugin))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(PlayerActions::new())
        .add_systems(Update, map_input_to_player_actions)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_all_objects, rotate_all_objects))
        .add_systems(Update, camera_deadzone_follow)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);
    // Start with level 1
    Level1::create().spawn(commands, asset_server);
}
