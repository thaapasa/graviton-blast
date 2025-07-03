use crate::background::components::BackgroundTile;
use crate::background::ParallaxBackground;
use bevy::asset::AssetServer;
use bevy::prelude::*;

/// Spawns player ship at the given position
pub fn spawn_parallax_background(
    commands: &mut Commands,
    asset_server: &AssetServer,
    background: ParallaxBackground,
) {
    let texture = background.sprite.load(asset_server);
    spawn_tile(commands, &background, 0, 0, Vec2::ZERO, texture.clone());
    commands.spawn(background);
}

pub fn spawn_tile(
    commands: &mut Commands,
    background: &ParallaxBackground,
    x: usize,
    y: usize,
    initial_pos: Vec2,
    tile_texture: Handle<Image>,
) {
    commands.spawn((
        Sprite::from_image(tile_texture),
        BackgroundTile { x, y },
        background.id,
        background.sprite.initial_transform(initial_pos),
    ));
}
