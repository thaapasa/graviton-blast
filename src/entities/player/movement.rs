use bevy::prelude::*;

#[derive(Resource)]
pub struct Movement {
    /// Current velocity of the ship.
    pub velocity: Vec2,
    /// Direction of thrust.
    pub direction: Vec2,
    /// Force of thrust. When zero, no acceleration is applied.
    pub force: f32,
}

impl Movement {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
            direction: Vec2::new(1.0, 0.7).normalize(),
            force: 150.0,
        }
    }

    pub fn update(&mut self, delta_secs: f32) {
        if self.force != 0.0 {
            self.velocity += self.direction * self.force * delta_secs;
        }
    }
}

pub fn accelerate_ship(time: Res<Time>, mut movement: ResMut<Movement>) {
    movement.update(time.delta_secs());
}
