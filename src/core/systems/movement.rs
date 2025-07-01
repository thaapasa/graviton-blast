use crate::assets::GameSprite;
use crate::core::components::{FacingAngle, Velocity};
use bevy::prelude::*;

pub fn move_all_objects(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        let delta = **velocity * time.delta_secs();
        transform.translation += delta.extend(0.0);
    }
}

pub fn rotate_all_objects(mut query: Query<(&mut Transform, &FacingAngle)>) {
    for (mut transform, facing_angle) in &mut query {
        transform.rotation = GameSprite::sprite_rotation(*facing_angle);
    }
}
