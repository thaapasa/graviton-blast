use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct TrailParticle {
    pub lifetime: f32,
}
