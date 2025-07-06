use bevy::prelude::*;

use crate::background::BackgroundPlugin;
use crate::black_hole::BlackHolePlugin;
use crate::constants::FIXED_UPDATE_RATE_HZ;
use crate::core::resources::PlayerActions;
use crate::core::systems::{
    accelerate_objects, camera_deadzone_follow, limit_velocity, map_input_to_player_actions,
    move_all_objects, quit_if_requested, rotate_all_objects, rotate_to_match_velocity,
};
use crate::core::UpdateSet;
use crate::enemy_ship::EnemyShipPlugin;
use crate::level::Level1;
use crate::player_ship::PlayerShipPlugin;
use crate::projectile::ProjectilePlugin;

mod assets;
mod background;
mod black_hole;
mod constants;
mod core;
mod dart_cloud;
mod enemy_ship;
mod level;
mod player_ship;
mod projectile;
mod utils;

#[cfg(test)]
pub mod tests;

fn main() {
    App::new()
        .configure_sets(Update, UpdateSet::schedule())
        .insert_resource(Time::<Fixed>::from_hz(FIXED_UPDATE_RATE_HZ))
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
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                map_input_to_player_actions.in_set(UpdateSet::Planning),
                rotate_to_match_velocity.in_set(UpdateSet::Movement),
                accelerate_objects.in_set(UpdateSet::Movement),
                move_all_objects.in_set(UpdateSet::Movement),
                rotate_all_objects.in_set(UpdateSet::Movement),
                limit_velocity.in_set(UpdateSet::PostMovement),
                quit_if_requested.in_set(UpdateSet::Finalize),
            ),
        )
        .add_systems(
            Update,
            camera_deadzone_follow.in_set(UpdateSet::PostMovement),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);
    // Start with level 1
    Level1::create().spawn(commands, asset_server);
}
