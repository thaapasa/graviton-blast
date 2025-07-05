use std::f32::consts::{FRAC_PI_2, PI};

use crate::core::components::Mass;

// Radians per second
pub const PLAYER_ROTATION_SPEED: f32 = PI + FRAC_PI_2;
pub const PLAYER_FORWARD_THRUST: f32 = 350.0;
pub const PLAYER_BACKWARD_THRUST: f32 = -250.0;

pub const PLAYER_SHIP_MASS: Mass = Mass::tons(3_000.0);
pub const ENEMY_SHIP_1_MASS: Mass = Mass::tons(4_500.0);
pub const BLACK_HOLE_MASS: Mass = Mass::tons(2_000_000_000.0);
pub const PROJECTILE_BLASTER_MASS: Mass = Mass::kg(100.0);

pub const MIN_DISTANCE_SQ: f32 = 0.00000001;
pub const GRAVITATIONAL_CONSTANT: f32 = 1.0e-5;
pub const MAX_GRAVITY_FORCE: f32 = 1000.0;
