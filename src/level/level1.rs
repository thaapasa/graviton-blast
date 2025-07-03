use crate::background::ParallaxBackground;
use crate::core::components::FacingAngle;
use crate::level::level_data::{LevelData, SpawnInfo};

pub struct Level1;

impl Level1 {
    pub fn create() -> LevelData {
        LevelData {
            background: ParallaxBackground::default_bg(),
            player_start: SpawnInfo::new(0.0, 0.0, FacingAngle::UP),
            black_holes: vec![SpawnInfo::new(300.0, 20.0, FacingAngle::default())],
        }
    }
}
