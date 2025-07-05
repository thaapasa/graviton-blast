use crate::constants::ENEMY_SHIP_1_MAX_THRUST;
use crate::core::components::{FacingAngle, PlayerShip, Thrust};
use crate::enemy_ship::components::EnemyShip;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_4;

pub fn move_enemy_ships(
    time: Res<Time>,
    player_query: Query<&Transform, With<PlayerShip>>,
    mut enemy_query: Query<(&Transform, &mut FacingAngle, &mut Thrust), With<EnemyShip>>,
) {
    let player_pos = player_query.single().unwrap().translation.truncate();
    for (transform, mut facing_angle, mut thrust) in &mut enemy_query {
        // Direction towards player
        let to_player = player_pos - transform.translation.truncate();
        if to_player == Vec2::ZERO {
            continue;
        }
        let to_player = to_player.normalize().to_angle();

        // Rotate to face player
        *facing_angle = facing_angle.rotate_towards(to_player, 3.0 * time.delta_secs());

        let angle_diff = facing_angle.angle_diff(to_player).abs();
        **thrust = if angle_diff < FRAC_PI_4 {
            // Thrust towards player
            (FRAC_PI_4 - angle_diff) * ENEMY_SHIP_1_MAX_THRUST
        } else {
            0.0
        }
    }
}
