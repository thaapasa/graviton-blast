use crate::core::components::{MaxVelocity, Thrust};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Dart;

pub struct DartCloud;

impl DartCloud {
    pub const SEPARATION: f32 = 200.0;
    pub const SEPARATION_SQUARED: f32 = DartCloud::SEPARATION * DartCloud::SEPARATION;
    pub const MAX_VELOCITY: MaxVelocity = MaxVelocity::new(400.0);
    pub const MAX_THRUST: Thrust = Thrust(2000.0);
    pub const ALIGNMENT_WEIGHT: f32 = 0.4;
    pub const TARGET_PLAYER_DISTANCE: f32 = 2000.0;
    pub const MAX_PLAYER_VEC_WEIGHT: f32 = 0.05;
    pub const TARGET_PLAYER_DISTANCE_SQUARED: f32 =
        DartCloud::TARGET_PLAYER_DISTANCE * DartCloud::TARGET_PLAYER_DISTANCE;
    pub const NEIGHBORS_TO_CALC: usize = 8;
}
