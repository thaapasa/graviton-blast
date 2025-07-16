use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::background::ParallaxBackground;
use crate::background::components::BackgroundTile;
use crate::core::components::FacingAngle;

pub fn spawn_parallax_background(
    commands: &mut Commands,
    asset_server: &AssetServer,
    background: ParallaxBackground,
) {
    let texture = background.sprite.load(asset_server);
    spawn_tile(commands, &background, 0, 0, texture.clone());
    commands.spawn(background);
}

/// Spawns a single background tile to be used for parallax background scroller.
/// Note: This just spawns the tile at location (0, 0), since we have a location
/// updater that moves every background tile to its correct location on each update.
pub fn spawn_tile(
    commands: &mut Commands,
    background: &ParallaxBackground,
    x: usize,
    y: usize,
    tile_texture: Handle<Image>,
) {
    commands.spawn((
        Name::new(format!("BackgroundTile {x}x{y}")),
        BackgroundTile { x, y },
        Sprite::from_image(tile_texture),
        background.id,
        background
            .sprite
            .initial_transform(Vec2::ZERO, FacingAngle::UP),
    ));
}
