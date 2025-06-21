use crate::entity::{black_hole, player};
use bevy::app::App;
use bevy::math::Vec2;

pub fn add_systems(app: &mut App) {
    player::add_systems(app, Vec2::new(0.0, 0.0));
    black_hole::add_systems(app, Vec2::new(150.0, -20.0))
}
