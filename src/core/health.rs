use bevy::prelude::*;

#[derive(Default, Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Health(f32);

impl Health {
    pub fn new(health: f32) -> Self {
        Self(health)
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_alive(&self) -> bool {
        self.0 > 0.0
    }

    #[inline]
    #[allow(dead_code)]
    pub fn take_damage(&mut self, damage: f32) {
        self.0 = (self.0 - damage).max(0.0);
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Default, Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxHealth(f32);

impl MaxHealth {
    pub fn new(health: f32) -> Self {
        Self(health)
    }

    #[inline]
    pub fn value(&self) -> f32 {
        self.0
    }
}
