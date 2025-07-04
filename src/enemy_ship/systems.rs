use crate::core::components::{FacingAngle, PlayerShip, Velocity};
use crate::enemy_ship::components::EnemyShip;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_4;

pub fn move_enemy_ships(
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerShip>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &mut FacingAngle), With<EnemyShip>>,
) {
    let player_pos = player_query.single().unwrap().translation.truncate();
    for (transform, mut velocity, mut facing_angle) in &mut enemy_query {
        // Direction towards player
        let to_player = player_pos - transform.translation.truncate();
        if to_player == Vec2::ZERO {
            continue;
        }
        let to_player = to_player.normalize().to_angle();

        // Rotate to face player
        *facing_angle = facing_angle.rotate_towards(to_player, 3.0 * time.delta_secs());

        if facing_angle.angle_diff(to_player).abs() < FRAC_PI_4 {
            // Thrust towards player
            *velocity += *facing_angle.to_velocity(400.0 * time.delta_secs());
        }
    }
}
