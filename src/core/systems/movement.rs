use crate::assets::GameSprite;
use crate::core::components::{FacingAngle, Velocity};
use bevy::prelude::*;

pub fn move_all_objects(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &FacingAngle)>,
) {
    for (mut transform, velocity, facing_angle) in &mut query {
        let delta = **velocity * time.delta_secs();
        transform.translation += delta.extend(0.0);
        transform.rotation = GameSprite::sprite_rotation(*facing_angle);
    }
}
