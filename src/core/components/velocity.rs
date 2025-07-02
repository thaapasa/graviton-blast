use bevy::prelude::*;
use std::ops::{Add, Deref, DerefMut};

#[derive(Component, Debug, Copy, Clone, Default, PartialEq)]
pub struct Velocity(Vec2);

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
}

impl Add for Velocity {
    type Output = Velocity;
    fn add(self, rhs: Self) -> Self::Output {
        Velocity(self.0 + rhs.0)
    }
}
