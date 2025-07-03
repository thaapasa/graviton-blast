use crate::assets::GameSprite;
use crate::black_hole::components::BlackHole;
use crate::core::components::Mass;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy::sprite::Sprite;

const BLACK_HOLE_MASS: Mass = Mass::tons(2_000_000_000.0);

pub fn spawn_black_hole(commands: &mut Commands, asset_server: &AssetServer, position: Vec2) {
    commands.spawn((
        BlackHole,
        Sprite::from_image(GameSprite::BlackHole.load(asset_server)),
        GameSprite::BlackHole.initial_transform(position),
        BLACK_HOLE_MASS,
    ));
}
