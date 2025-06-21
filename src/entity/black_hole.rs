use bevy::app::{App, Startup};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{default, Commands, Sprite, Transform};

#[allow(clippy::module_inception)]
mod black_hole;

pub fn create_setup(starting_pos: Vec2) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        commands.spawn((
            Sprite {
                color: Color::srgba(0.8, 0.2, 0.2, 1.0),
                custom_size: Some(Vec2::splat(3.0)), // Adjust size as needed
                ..default()
            },
            Transform::from_translation(starting_pos.extend(1.0)),
        ));
    }
}

pub fn add_systems(app: &mut App, starting_pos: Vec2) {
    app.add_systems(Startup, create_setup(starting_pos));
}
