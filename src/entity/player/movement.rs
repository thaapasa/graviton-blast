use crate::data::{Direction, Rotation, Thrust};
use crate::entity::player::player_actions::PlayerActions;
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};

const ROTATION_SPEED_RADIANS_PER_SEC: f32 = PI + FRAC_PI_2;
const FWD_THRUST: f32 = 350.0;
const BWD_THRUST: f32 = -250.0;

#[derive(Resource)]
pub struct Movement {
    /// Current velocity of the ship.
    pub velocity: Vec2,
    /// Direction of thrust (angle, in radians).
    pub direction: Direction,
    /// Force of thrust. When zero, no acceleration is applied.
    pub force: Thrust,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
            direction: Direction::default(),
            force: Thrust::default(),
        }
    }

    pub fn update(&mut self, delta_secs: f32) {
        if self.force.has_thrust() {
            self.velocity += self.direction.as_vec(self.force.0 * delta_secs);
        }
    }
}

pub fn accelerate_ship(time: Res<Time>, mut movement: ResMut<Movement>) {
    movement.update(time.delta_secs());
}

pub fn update_movement(
    time: Res<Time>,
    actions: Res<PlayerActions>,
    mut movement: ResMut<Movement>,
) {
    let elapsed = time.delta_secs();
    movement.force = match actions.thrust {
        Some(true) => Thrust(FWD_THRUST),
        Some(false) => Thrust(BWD_THRUST),
        None => movement.force,
    };
    match actions.rotate {
        Some(Rotation::Clockwise) => movement.direction -= ROTATION_SPEED_RADIANS_PER_SEC * elapsed,
        Some(Rotation::Anticlockwise) => {
            movement.direction += ROTATION_SPEED_RADIANS_PER_SEC * elapsed
        }
        _ => (),
    }
}
