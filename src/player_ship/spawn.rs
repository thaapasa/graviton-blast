use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::assets::GameSprite;
use crate::constants::{PLAYER_SHIP_MASS, PLAYER_SHIP_MAX_VELOCITY};
use crate::core::components::{FacingAngle, PlayerShip, Thrust, Velocity};
use crate::core::health::{Health, MaxHealth};

/// Spawns player ship at the given position
pub fn spawn_player_ship(
    commands: &mut Commands,
    asset_server: &AssetServer,
    starting_pos: Vec2,
    facing_angle: FacingAngle,
) {
    commands.spawn((
        Name::new("PlayerShip"),
        PlayerShip,
        Sprite::from_image(GameSprite::PlayerShip.load(asset_server)),
        GameSprite::PlayerShip.initial_transform(starting_pos, facing_angle),
        facing_angle,
        Thrust::ZERO,
        Velocity::ZERO,
        PLAYER_SHIP_MASS,
        PLAYER_SHIP_MAX_VELOCITY,
        Health::new(100.0),
        MaxHealth::new(100.0),
    ));
}
