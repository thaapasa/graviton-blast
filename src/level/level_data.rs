use bevy::prelude::*;

use crate::background::ParallaxBackground;
use crate::core::SpawnInfo;
use crate::enemy_ship::EnemyShipType;

#[derive(Debug, Clone)]
pub struct LevelData {
    pub background: Vec<ParallaxBackground>,
    pub player_start: SpawnInfo,
    pub black_holes: Vec<SpawnInfo>,
    pub enemies: Vec<(EnemyShipType, SpawnInfo)>,
}
