use crate::core::components::{Thrust, Velocity};
use crate::core::SpawnInfo;
use crate::enemy_ship::components::{EnemyShip, EnemyShipType};
use bevy::prelude::*;

pub fn spawn_enemy_ship(
    commands: &mut Commands,
    asset_server: &AssetServer,
    enemy_type: EnemyShipType,
    spawn_info: SpawnInfo,
) {
    let sprite = enemy_type.sprite();
    commands.spawn((
        EnemyShip,
        Sprite::from_image(sprite.load(asset_server)),
        Velocity::ZERO,
        spawn_info.angle,
        sprite.initial_transform(spawn_info.as_location(), spawn_info.angle),
        Thrust::ZERO,
        enemy_type.mass(),
        enemy_type.max_velocity(),
    ));
}
