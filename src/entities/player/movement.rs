use bevy::prelude::*;

#[derive(Resource)]
pub struct Movement {
    pub direction: Vec2,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            direction: Vec2::new(1.0, 0.7).normalize() * 150.0,
        }
    }
}
