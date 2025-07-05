use bevy::prelude::*;

use crate::assets::GameSprite;
use crate::core::components::{AngleFollowsVelocity, FacingAngle, MaxVelocity, Velocity};
use crate::utils::Vec2Ext;

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

pub fn rotate_to_match_velocity(
    mut query: Query<(&mut FacingAngle, &Velocity), With<AngleFollowsVelocity>>,
) {
    for (mut facing_angle, velocity) in &mut query {
        if let Some(angle) = velocity.angle() {
            *facing_angle = angle;
        }
    }
}

pub fn limit_velocity(mut query: Query<(&mut Velocity, &MaxVelocity)>) {
    for (mut velocity, max_velocity) in &mut query {
        **velocity = velocity.clamp_max_length_squared(max_velocity.squared());
    }
}
