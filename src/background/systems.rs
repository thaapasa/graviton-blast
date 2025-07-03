use crate::background::components::{BackgroundTile, ParallaxLayerId};
use crate::background::spawn::spawn_tile;
use crate::background::ParallaxBackground;
use crate::utils::{Vec2Ext, VecExt};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use std::collections::{BTreeMap, BTreeSet};

pub fn relocate_parallax_background(
    camera_query: Query<&Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    query_bg: Query<&ParallaxBackground>,
    mut query: Query<(&ParallaxLayerId, &BackgroundTile, &mut Transform), Without<Camera>>,
) {
    let camera = camera_query.single().unwrap();
    let window_size = window_query.single().unwrap().size();
    let bg_map = query_bg
        .into_iter()
        .map(|b| (b.id, b))
        .collect::<BTreeMap<_, _>>();
    for (layer_id, tile, mut transform) in &mut query {
        let bg = bg_map.get(layer_id).unwrap();
        let pos = bg.get_tile_position(camera.translation.truncate(), window_size, tile);
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

pub fn on_resize_window(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut resize_events: EventReader<WindowResized>,
    query_bg: Query<&ParallaxBackground>,
    query_tiles: Query<(Entity, &ParallaxLayerId, &BackgroundTile)>,
) {
    for event in resize_events.read() {
        let backgrounds = query_bg.iter().collect::<Vec<_>>();

        let grouped = query_tiles.iter().collect::<Vec<_>>().group_by(|x| x.1);

        for bg in backgrounds {
            let required_tiles = bg.required_tiles(event.width, event.height);
            let mut seen_tiles = BTreeSet::<(usize, usize)>::new();
            let tiles = grouped.get(&bg.id).unwrap_or(&NO_TILES);
            for (_entity, _layer_id, tile) in tiles {
                seen_tiles.insert((tile.x, tile.y));
            }
            for x in 0..required_tiles.0 {
                for y in 0..required_tiles.1 {
                    if seen_tiles.contains(&(x, y)) {
                        continue;
                    }
                    // Add missing tile
                    spawn_tile(
                        &mut commands,
                        bg,
                        x,
                        y,
                        Vec2::ZERO,
                        bg.sprite.load(&asset_server),
                    );
                }
            }
        }
    }
}

static NO_TILES: Vec<(Entity, &ParallaxLayerId, &BackgroundTile)> = vec![];

impl ParallaxBackground {
    /// Returns the number of `(horizontal, vertical)` tiles required to cover
    /// a screen of the given size.
    fn required_tiles(&self, screen_width: f32, screen_height: f32) -> (usize, usize) {
        (
            self.required_tiles_for_side(screen_width),
            self.required_tiles_for_side(screen_height),
        )
    }

    #[inline]
    fn required_tiles_for_side(&self, dimension: f32) -> usize {
        if dimension <= 0.0 {
            return 0;
        }
        if dimension <= self.gap {
            return 1;
        }
        let required = 2.0 * dimension / self.tile_size();
        required.ceil() as usize
    }

    #[inline]
    fn tile_size(&self) -> f32 {
        self.sprite.scaled_size() + self.gap
    }

    #[inline]
    fn get_tile_position(
        &self,
        camera_pos: Vec2,
        screen_size: Vec2,
        tile: &BackgroundTile,
    ) -> Vec2 {
        self.bottom_left_tile_position(camera_pos, screen_size)
            + Vec2::new(tile.x as f32, tile.y as f32) * self.tile_size()
    }

    /// Returns the position within one tile that should be visible in the bottom-left
    /// corner of the visible screen. This location is guaranteed to fit inside one tile.
    #[inline]
    fn bottom_left_point(&self, camera_pos: Vec2, screen_size: Vec2) -> Vec2 {
        (camera_pos * self.speed - screen_size / 2.0 + self.offset)
            .rem_euclid_scalar(self.tile_size())
    }

    #[inline]
    fn bottom_left_tile_position(&self, camera_pos: Vec2, screen_size: Vec2) -> Vec2 {
        let shown_point = self.bottom_left_point(camera_pos, screen_size);
        camera_pos - screen_size / 2.0 - shown_point + self.tile_size() / 2.0
    }
}

#[cfg(test)]
mod tests {
    use crate::assets::GameSprite;
    use crate::background::ParallaxBackground;
    use bevy::prelude::Vec2;

    #[test]
    fn test_required_tiles_no_gap() {
        let bg = ParallaxBackground {
            sprite: GameSprite::StarsSparse,
            gap: 0.0,
            ..Default::default()
        };

        // Side width = 1024

        assert_eq!(bg.required_tiles_for_side(0.0), 0);
        assert_eq!(bg.required_tiles_for_side(1.0), 1);
        assert_eq!(bg.required_tiles_for_side(100.0), 1);
        assert_eq!(bg.required_tiles_for_side(511.0), 1);
        assert_eq!(bg.required_tiles_for_side(512.0), 1);
        assert_eq!(bg.required_tiles_for_side(1020.0), 2);
        assert_eq!(bg.required_tiles_for_side(1024.0), 2);
        assert_eq!(bg.required_tiles_for_side(1024.1), 3);
        assert_eq!(bg.required_tiles_for_side(1030.0), 3);
        assert_eq!(bg.required_tiles_for_side(2040.0), 4);
        assert_eq!(bg.required_tiles_for_side(2049.0), 5);
    }

    #[test]
    fn test_required_tiles_with_gap() {
        let bg = ParallaxBackground {
            sprite: GameSprite::StarsSparse,
            gap: 100.0,
            ..Default::default()
        };

        assert_eq!(bg.required_tiles_for_side(0.0), 0);
        assert_eq!(bg.required_tiles_for_side(1.0), 1);
        assert_eq!(bg.required_tiles_for_side(100.0), 1);
        assert_eq!(bg.required_tiles_for_side(513.0), 1);
        assert_eq!(bg.required_tiles_for_side(562.0), 1);
        assert_eq!(bg.required_tiles_for_side(563.0), 2);
        assert_eq!(bg.required_tiles_for_side(1124.0), 2);
        assert_eq!(bg.required_tiles_for_side(1126.0), 3);
    }

    #[test]
    fn test_bottom_left_point() {
        let bg = ParallaxBackground {
            sprite: GameSprite::StarsSparse,
            ..Default::default()
        };
        let screen_size = Vec2::new(1000.0, 1000.0);
        assert_eq!(
            bg.bottom_left_point(Vec2::new(0.0, 0.0), screen_size),
            Vec2::new(524.0, 524.0)
        );
        assert_eq!(
            bg.bottom_left_point(Vec2::new(1.0, 2.0), screen_size),
            Vec2::new(525.0, 526.0)
        );
        assert_eq!(
            bg.bottom_left_point(Vec2::new(-1.0, -2.0), screen_size),
            Vec2::new(523.0, 522.0)
        );
        assert_eq!(
            bg.bottom_left_point(Vec2::new(501.0, 499.0), screen_size),
            Vec2::new(1.0, 1023.0)
        );
    }
}
