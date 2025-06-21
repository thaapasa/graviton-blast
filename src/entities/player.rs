use crate::entities::player::movement::accelerate_ship;
use crate::entities::player::ship::{move_ball, Ship};
use crate::entities::player::trail::{fade_particles, spawn_trail_particles};
use bevy::app::{App, Startup};
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Commands, Resource, Sprite, Transform, Update};

pub mod movement;
pub mod ship;
pub mod trail;

pub fn create_resource() -> impl Resource {
    movement::Movement::new()
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(2.0)), // Adjust size as needed
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        Ship,
    ));
}

pub fn add_systems(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            accelerate_ship,
            move_ball,
            spawn_trail_particles,
            fade_particles,
        ),
    );
}
