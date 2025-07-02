use crate::assets::GameSprite;
use bevy::prelude::Component;

/// Repeating background tile, rendered with a parallax effect against the camera
#[derive(Component, Debug, Clone)]
pub struct ParallaxBackground {
    /// Sprite to use to draw the background
    pub sprite: GameSprite,
    /// Gap between each tile of the background
    #[allow(dead_code)]
    pub gap: f32,
    /// Scrolling speed
    #[allow(dead_code)]
    pub speed: f32,
}

impl ParallaxBackground {
    pub fn default_bg() -> Vec<ParallaxBackground> {
        vec![
            ParallaxBackground {
                sprite: GameSprite::StarsSparse,
                gap: 0.0,
                speed: 1.0,
            },
            ParallaxBackground {
                sprite: GameSprite::StarsLarge,
                gap: 0.0,
                speed: 1.5,
            },
        ]
    }
}
