use crate::enemy_ship::systems::move_enemy_ships;
use bevy::prelude::*;

pub struct EnemyShipPlugin;

impl Plugin for EnemyShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_enemy_ships);
    }
}
