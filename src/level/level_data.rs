use crate::background::ParallaxBackground;
use crate::core::components::FacingAngle;
use bevy::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SpawnInfo {
    pub x: f32,
    pub y: f32,
    pub angle: FacingAngle,
}

impl SpawnInfo {
    pub fn new(x: f32, y: f32, angle: impl Into<FacingAngle>) -> SpawnInfo {
        SpawnInfo {
            x,
            y,
            angle: angle.into(),
        }
    }
    pub fn as_location(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct LevelData {
    pub background: Vec<ParallaxBackground>,
    pub player_start: SpawnInfo,
    pub black_holes: Vec<SpawnInfo>,
}
