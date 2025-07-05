use bevy::prelude::*;

use crate::black_hole::components::BlackHole;
use crate::constants::{GRAVITATIONAL_CONSTANT, MAX_GRAVITY_FORCE, MIN_DISTANCE_SQ};
use crate::core::components::{Mass, Velocity};

pub fn apply_gravity(
    time: Res<Time>,
    bh_query: Query<(&Transform, &Mass), With<BlackHole>>,
    mut object_query: Query<(&mut Velocity, &Mass, &Transform), Without<BlackHole>>,
) {
    for (bh_position, bh_mass) in &bh_query {
        for (mut velocity, _mass, position) in &mut object_query {
            let dir = (bh_position.translation - position.translation).truncate();
            let distance_squared = dir.length_squared().max(MIN_DISTANCE_SQ);

            // Ship mass cancels out
            let force_magnitude = (GRAVITATIONAL_CONSTANT * bh_mass.value() / distance_squared)
                .min(MAX_GRAVITY_FORCE);
            let acceleration = dir.normalize() * force_magnitude;

            *velocity += acceleration * time.delta_secs();
        }
    }
}
