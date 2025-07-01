use crate::core::components::FacingAngle;
use crate::level::level_data::{LevelData, SpawnInfo};

pub struct Level1;

impl Level1 {
    pub fn create() -> LevelData {
        LevelData {
            player_start: SpawnInfo {
                x: 0.0,
                y: 0.0,
                angle: FacingAngle::UP,
            },
            black_holes: vec![SpawnInfo {
                x: 40.0,
                y: 20.0,
                ..Default::default()
            }],
        }
    }
}
