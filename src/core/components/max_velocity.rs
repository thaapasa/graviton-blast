use bevy::prelude::Component;

/// Assign to objects to automatically limit their velocity magnitude to the given value.
#[derive(Debug, Component, Clone, Copy)]
pub struct MaxVelocity(f32, f32);

impl MaxVelocity {
    pub const fn new(max_velocity: f32) -> Self {
        Self(max_velocity, max_velocity * max_velocity)
    }

    pub fn squared(&self) -> f32 {
        self.1
    }

    pub fn velocity(&self) -> f32 {
        self.0
    }
}
