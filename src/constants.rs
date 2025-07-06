use crate::core::components::{Mass, MaxVelocity};
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;

/// Golden angle = `π * (3 - √5)` radians
#[allow(clippy::excessive_precision)]
pub const GOLDEN_ANGLE: f32 = 137.5077640500378546463487f32;

pub const FIXED_UPDATE_RATE_HZ: f64 = 60.0;
pub const FPS_REFRESH_INTERVAL: Duration = Duration::from_millis(330);

// Radians per second
pub const PLAYER_ROTATION_SPEED: f32 = PI + FRAC_PI_2;
pub const PLAYER_FORWARD_THRUST: f32 = 350.0;
pub const PLAYER_BACKWARD_THRUST: f32 = -250.0;

pub const ENEMY_SHIP_1_MAX_THRUST: f32 = 800.0;

pub const PLAYER_SHIP_MASS: Mass = Mass::tons(3_000.0);
pub const ENEMY_SHIP_1_MASS: Mass = Mass::tons(4_500.0);
pub const BLACK_HOLE_MASS: Mass = Mass::tons(2_000_000_000.0);
pub const PROJECTILE_BLASTER_MASS: Mass = Mass::kg(100.0);

pub const PLAYER_SHIP_MAX_VELOCITY: MaxVelocity = MaxVelocity::new(1000.0);
pub const ENEMY_SHIP_1_MAX_VELOCITY: MaxVelocity = MaxVelocity::new(800.0);

pub const MIN_DISTANCE_SQ: f32 = 0.00000001;
pub const GRAVITATIONAL_CONSTANT: f32 = 1.0e-5;
pub const MAX_GRAVITY_FORCE: f32 = 1000.0;
