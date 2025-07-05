use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::Commands;
use bevy::sprite::Sprite;

use crate::assets::GameSprite;
use crate::black_hole::components::BlackHole;
use crate::constants::BLACK_HOLE_MASS;

pub fn spawn_black_hole(commands: &mut Commands, asset_server: &AssetServer, position: Vec2) {
    commands.spawn((
        BlackHole,
        Sprite::from_image(GameSprite::BlackHole.load(asset_server)),
        GameSprite::BlackHole.initial_transform(position),
        BLACK_HOLE_MASS,
    ));
}
