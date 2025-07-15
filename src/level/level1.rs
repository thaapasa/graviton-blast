use crate::background::ParallaxBackground;
use crate::core::SpawnInfo;
use crate::core::components::FacingAngle;
use crate::enemy_ship::EnemyShipType::Enemy1;
use crate::level::level_data::LevelData;

pub struct Level1;

impl Level1 {
    pub fn create() -> LevelData {
        LevelData {
            background: ParallaxBackground::default_bg(),
            player_start: SpawnInfo::new(-400.0, -100.0, FacingAngle::UP),
            black_holes: vec![SpawnInfo::new(300.0, 20.0, FacingAngle::default())],
            enemies: vec![(Enemy1, SpawnInfo::new(100.0, 100.0, FacingAngle::DOWN))],
            dart_clouds: vec![(SpawnInfo::new(-200.0, 50.0, FacingAngle::default()), 100)],
        }
    }
}
