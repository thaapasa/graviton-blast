use crate::background::ParallaxBackground;
use bevy::asset::AssetServer;
use bevy::prelude::*;

/// Spawns player ship at the given position
pub fn spawn_parallax_background(
    commands: &mut Commands,
    asset_server: &AssetServer,
    background: ParallaxBackground,
) {
    commands.spawn((
        Sprite::from_image(background.sprite.load(asset_server)),
        background.sprite.initial_transform(Vec2::ZERO),
    ));
}
