mod entities;

use crate::entities::player::ship::{move_ball, Movement, Ship};
use crate::entities::player::trail::{fade_particles, spawn_trail_particles};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Movement {
            direction: Vec2::new(1.0, 0.7).normalize() * 150.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_ball, spawn_trail_particles, fade_particles))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

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
