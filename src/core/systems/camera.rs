use crate::player_ship::PlayerShip;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn camera_deadzone_follow(
    ship_query: Query<&Transform, With<PlayerShip>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerShip>)>,
) {
    let window = window_query.single().unwrap();
    let ship_transform = ship_query.single().unwrap();
    let mut camera_transform = camera_query.single_mut().unwrap();

    let deadzone_half_size = window.size() * 0.8 / 2.0;
    let ship_pos = ship_transform.translation.truncate();
    let cam_pos = camera_transform.translation.truncate();

    let delta = ship_pos - cam_pos;
    let mut cam_move = Vec2::ZERO;

    if delta.x.abs() > deadzone_half_size.x {
        cam_move.x = delta.x - deadzone_half_size.x * delta.x.signum();
    }
    if delta.y.abs() > deadzone_half_size.y {
        cam_move.y = delta.y - deadzone_half_size.y * delta.y.signum();
    }

    camera_transform.translation.x += cam_move.x;
    camera_transform.translation.y += cam_move.y;
}
