use bevy::prelude::*;

use crate::constants::{PLAYER_BACKWARD_THRUST, PLAYER_FORWARD_THRUST, PLAYER_ROTATION_SPEED};
use crate::core::components::{FacingAngle, PlayerShip, Thrust, Velocity};
use crate::core::player_actions::PlayerActions;
use crate::core::Rotation;
use crate::projectile::{spawn_projectile, ProjectileType};

/// Updates player ship movement based on player actions
pub fn update_player_movement(
    time: Res<Time>,
    actions: Res<PlayerActions>,
    mut query: Query<(&mut Thrust, &mut FacingAngle), With<PlayerShip>>,
) {
    let elapsed = time.delta_secs();
    let (mut thrust, mut facing_angle) = query.single_mut().unwrap();

    *thrust = match actions.thrust {
        Some(true) => Thrust(PLAYER_FORWARD_THRUST),
        Some(false) => Thrust(PLAYER_BACKWARD_THRUST),
        None => Thrust::ZERO,
    };
    match actions.rotate {
        Some(Rotation::Clockwise) => *facing_angle -= PLAYER_ROTATION_SPEED * elapsed,
        Some(Rotation::Anticlockwise) => *facing_angle += PLAYER_ROTATION_SPEED * elapsed,
        _ => (),
    }
}

/// Fire player weapons if requested
pub fn fire_player_weapons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    actions: Res<PlayerActions>,
    query: Query<(&Velocity, &FacingAngle, &Transform), With<PlayerShip>>,
) {
    if actions.fire {
        let (velocity, facing_angle, transform) = query.single().unwrap();
        fire_blaster(
            &mut commands,
            &asset_server,
            transform.translation.truncate(),
            *velocity,
            *facing_angle,
        );
    }
}

pub fn fire_blaster(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    velocity: Velocity,
    facing_angle: FacingAngle,
) {
    for i in [-1.0, 1.0] {
        let offset = Vec2::new(0.0, i * 13.0);
        spawn_projectile(
            commands,
            asset_server,
            ProjectileType::PlayerBlaster,
            position + facing_angle.rotate(offset),
            velocity,
            facing_angle,
        );
    }
}
