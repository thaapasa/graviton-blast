use crate::assets::{DrawingOrder, GameSprite};
use crate::core::components::{FacingAngle, Thrust, Velocity};
use crate::player_ship::components::PlayerShip;
use bevy::asset::AssetServer;
use bevy::prelude::*;

/// Spawns player ship at the given position
pub fn spawn_player_ship(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    starting_pos: Vec2,
    facing_angle: FacingAngle,
) {
    let texture = GameSprite::PlayerShip.load(asset_server);
    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_translation(DrawingOrder::PlayerShip.to_vec_3d(starting_pos))
            .with_scale(Vec3::splat(GameSprite::PlayerShip.scale())),
        PlayerShip,
        facing_angle,
        Thrust::ZERO,
        Velocity::ZERO,
    ));
}
