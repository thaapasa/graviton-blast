use bevy::prelude::Component;
use std::ops::{Deref, DerefMut};

#[derive(Component, Debug, Copy, Clone, Default, PartialEq)]
pub struct Thrust(pub f32);

impl Thrust {
    #[allow(dead_code)]
    pub const ZERO: Thrust = Thrust(0.0);

    #[inline]
    pub fn has_thrust(&self) -> bool {
        self.0 != 0.0
    }
}

impl Deref for Thrust {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Thrust {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
