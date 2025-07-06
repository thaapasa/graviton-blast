use crate::core::components::FacingAngle;
use bevy::prelude::*;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div};

#[derive(Component, Debug, Copy, Clone, Default, PartialEq)]
pub struct Velocity(Vec2);

/// Velocity, that will be applied after the planning step.
/// This can be used to calculate velocity based on other entities velocity, in the planning step.
#[derive(Component, Debug, Default, Copy, Clone, PartialEq)]
pub struct NextVelocity(pub Vec2);

impl Deref for Velocity {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Velocity {
    pub const ZERO: Velocity = Velocity(Vec2::ZERO);

    #[inline]
    pub fn new(vec: Vec2) -> Self {
        Self(vec)
    }

    pub fn angle(&self) -> Option<FacingAngle> {
        if self.0 == Vec2::ZERO {
            return None;
        }
        Some(FacingAngle::new(self.0.to_angle()))
    }
}

impl Add for Velocity {
    type Output = Velocity;
    fn add(self, rhs: Self) -> Self::Output {
        Velocity(self.0 + rhs.0)
    }
}

impl AddAssign<Vec2> for Velocity {
    fn add_assign(&mut self, rhs: Vec2) {
        self.0 = self.0 + rhs
    }
}

impl Div<f32> for Velocity {
    type Output = Velocity;

    fn div(self, rhs: f32) -> Self::Output {
        Velocity(self.0 / rhs)
    }
}
