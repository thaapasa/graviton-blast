use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

use crate::core::components::{FacingAngle, PlayerShip, Thrust, Velocity};
use crate::core::resources::PlayerActions;
use crate::core::Rotation;
use crate::projectile::{spawn_projectile, ProjectileType};

const ROTATION_SPEED_RADIANS_PER_SEC: f32 = PI + FRAC_PI_2;
const FWD_THRUST: f32 = 350.0;
const BWD_THRUST: f32 = -250.0;

/// Updates player ship movement based on player actions
pub fn update_player_movement(
    time: Res<Time>,
    actions: Res<PlayerActions>,
    mut query: Query<(&mut Thrust, &mut FacingAngle), With<PlayerShip>>,
) {
    let elapsed = time.delta_secs();
    let (mut thrust, mut facing_angle) = query.single_mut().unwrap();

    *thrust = match actions.thrust {
        Some(true) => Thrust(FWD_THRUST),
        Some(false) => Thrust(BWD_THRUST),
        None => Thrust::ZERO,
    };
    match actions.rotate {
        Some(Rotation::Clockwise) => *facing_angle -= ROTATION_SPEED_RADIANS_PER_SEC * elapsed,
        Some(Rotation::Anticlockwise) => *facing_angle += ROTATION_SPEED_RADIANS_PER_SEC * elapsed,
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
        spawn_projectile(
            &mut commands,
            &asset_server,
            ProjectileType::PlayerBlaster,
            transform.translation.truncate(),
            *facing_angle,
            *velocity,
        );
    }
}

pub fn accelerate_player_ship(
    time: Res<Time>,
    mut vel_query: Query<&mut Velocity, With<PlayerShip>>,
    query: Query<(&Thrust, &FacingAngle), With<PlayerShip>>,
) {
    let elapsed = time.delta_secs();
    let (thrust, facing_angle) = query.single().unwrap();
    let mut velocity = vel_query.single_mut().unwrap();

    if thrust.has_thrust() {
        **velocity += facing_angle.as_vec(**thrust * elapsed);
    }
}
