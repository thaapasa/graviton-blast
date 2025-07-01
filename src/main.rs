use crate::core::resources::PlayerActions;
use crate::core::systems::map_input_to_player_actions;
use crate::level::Level1;
use crate::player_ship::PlayerShipPlugin;
use bevy::prelude::*;

mod assets;
mod core;
mod level;
mod player_ship;

#[cfg(test)]
pub mod tests;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerShipPlugin,))
        .insert_resource(PlayerActions::new())
        .add_systems(Update, map_input_to_player_actions)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);
    // Start with level 1
    Level1::create().spawn(commands, asset_server);
}
