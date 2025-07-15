use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use std::time::Duration;

use crate::background::BackgroundPlugin;
use crate::black_hole::BlackHolePlugin;
use crate::constants::FIXED_UPDATE_RATE_HZ;
use crate::core::components::SpatialTracked;
use crate::core::player_actions::{PlayerActions, map_input_to_player_actions, quit_if_requested};
use crate::core::systems::{
    accelerate_objects, camera_deadzone_follow, copy_next_velocity, limit_velocity,
    move_all_objects, rotate_all_objects, rotate_to_match_velocity,
};
use crate::core::{UpdateSet, fps};
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
pub use crate::dart_cloud::DartCloudPlugin;
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
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(FIXED_UPDATE_RATE_HZ))
        .insert_resource(PlayerActions::new())
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(
            AutomaticUpdate::<SpatialTracked>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_secs_f32(0.1))
                .with_transform(TransformMode::GlobalTransform),
        )
        .add_plugins((
            PlayerShipPlugin,
            EnemyShipPlugin,
            BackgroundPlugin,
            BlackHolePlugin,
            ProjectilePlugin,
            DartCloudPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                map_input_to_player_actions.in_set(UpdateSet::Planning),
                copy_next_velocity.in_set(UpdateSet::PlayerAction),
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
            (
                fps::update_fps.in_set(UpdateSet::Finalize),
                camera_deadzone_follow.in_set(UpdateSet::PostMovement),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    fps::spawn_fps_counter(&mut commands);

    // Start with level 1
    Level1::create().spawn(&mut commands, asset_server);
}
