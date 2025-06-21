use crate::entities::player::movement::Movement;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ship;

pub fn move_ball(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ship>>,
    mut movement: ResMut<Movement>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let mut transform = query.single_mut().unwrap();
    let delta = movement.velocity * time.delta_secs();
    transform.translation += delta.extend(0.0);

    // Bounce off window edges
    let bounds = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let pos = transform.translation;

    if pos.x.abs() > bounds.x {
        movement.direction.x *= -1.0;
    }
    if pos.y.abs() > bounds.y {
        movement.direction.y *= -1.0;
    }
}
