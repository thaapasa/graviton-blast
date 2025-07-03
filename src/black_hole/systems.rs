use crate::black_hole::components::BlackHole;
use crate::core::components::{Mass, Velocity};
use bevy::prelude::*;

const MIN_DISTANCE_SQ: f32 = 0.00000001;
const GRAVITATIONAL_CONSTANT: f32 = 1.0e-5;
const MAX_APPLIED_FORCE: f32 = 1000.0;

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
                .min(MAX_APPLIED_FORCE);
            let acceleration = dir.normalize() * force_magnitude;

            *velocity += acceleration * time.delta_secs();
        }
    }
}
