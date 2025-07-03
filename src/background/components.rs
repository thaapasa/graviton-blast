use bevy::prelude::*;
use std::sync::atomic::AtomicUsize;

use crate::assets::GameSprite;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParallaxLayerId(usize);

#[derive(Component, Debug, Clone)]
pub struct BackgroundTile {
    /// X coordinate of the tile, within the tiling layout.
    /// Range: ``[0, <required_horizontal_tiles)``
    pub x: usize,
    /// Y coordinate of the tile, within the tiling layout.
    /// Range: ``[0, required_vertical_tiles)``
    pub y: usize,
}
static NEXT_LAYER_ID: AtomicUsize = AtomicUsize::new(0);

impl ParallaxLayerId {
    pub fn new() -> Self {
        ParallaxLayerId(NEXT_LAYER_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
    }
    #[allow(dead_code)]
    pub fn value(&self) -> usize {
        self.0
    }
}

impl Default for ParallaxLayerId {
    fn default() -> Self {
        Self::new()
    }
}

/// Repeating background tile, rendered with a parallax effect against the camera
#[derive(Component, Debug, Clone)]
pub struct ParallaxBackground {
    /// Layer id, for grouping tiles that belong to the layer.
    /// Do not spawn multiple backgrounds with the same id.
    pub id: ParallaxLayerId,
    /// Sprite to use to draw the background.
    pub sprite: GameSprite,
    /// Gap, in logical screen pixels, between each tile of the background.
    /// Note: this is not scaled with the scaling of the background tile sprites.
    #[allow(dead_code)]
    pub gap: f32,
    /// Scrolling speed
    #[allow(dead_code)]
    pub speed: f32,
    /// Static offset to tiling position
    pub offset: Vec2,
}

impl ParallaxBackground {
    pub fn default_bg() -> Vec<ParallaxBackground> {
        vec![
            ParallaxBackground {
                sprite: GameSprite::StarsSparse,
                speed: 0.37,
                ..Default::default()
            },
            ParallaxBackground {
                sprite: GameSprite::StarsLarge,
                speed: 0.69,
                ..Default::default()
            },
        ]
    }
}

impl Default for ParallaxBackground {
    fn default() -> Self {
        Self {
            id: ParallaxLayerId::new(),
            sprite: GameSprite::StarsSparse,
            gap: 0.0,
            speed: 1.0,
            offset: Vec2::ZERO,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::background::components::ParallaxLayerId;

    #[test]
    fn test_layer_id() {
        let id1 = ParallaxLayerId::new();
        let id2 = ParallaxLayerId::new();
        assert_eq!(id2.value(), id1.value() + 1);
    }
}
