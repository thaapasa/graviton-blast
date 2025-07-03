use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::assets::GameSprite;
use crate::core::components::{FacingAngle, PlayerShip, Thrust, Velocity};

/// Spawns player ship at the given position
pub fn spawn_player_ship(
    commands: &mut Commands,
    asset_server: &AssetServer,
    starting_pos: Vec2,
    facing_angle: FacingAngle,
) {
    commands.spawn((
        PlayerShip,
        Sprite::from_image(GameSprite::PlayerShip.load(asset_server)),
        GameSprite::PlayerShip.initial_transform(starting_pos),
        facing_angle,
        Thrust::ZERO,
        Velocity::ZERO,
    ));
}
