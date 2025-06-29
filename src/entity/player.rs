use crate::entity::player::movement::{accelerate_ship, update_movement};
use crate::entity::player::player_actions::map_input_to_actions;
use crate::entity::player::ship::{move_ball, Ship};
use crate::entity::player::trail::{fade_particles, spawn_trail_particles};
use crate::images::sprite::GameImage;
use bevy::app::{App, Startup};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res, Sprite, Transform, Update};

pub mod movement;
mod player_actions;
pub mod ship;
pub mod trail;

pub fn insert_resources(app: &mut App) {
    app.insert_resource(player_actions::PlayerActions::new());
    app.insert_resource(movement::Movement::new());
}

pub fn create_setup(starting_pos: Vec2) -> impl Fn(Commands, Res<AssetServer>) {
    move |mut commands: Commands, asset_server: Res<AssetServer>| {
        let texture = GameImage::PlayerShip.load(asset_server);
        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_translation(starting_pos.extend(1.0)),
            Ship,
        ));
    }
}

pub fn add_systems(app: &mut App, starting_pos: Vec2) {
    app.add_systems(Startup, create_setup(starting_pos));
    app.add_systems(
        Update,
        (
            map_input_to_actions,
            update_movement,
            accelerate_ship,
            move_ball,
            spawn_trail_particles,
            fade_particles,
        ),
    );
}
