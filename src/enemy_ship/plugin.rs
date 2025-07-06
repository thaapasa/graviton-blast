use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::enemy_ship::systems::move_enemy_ships;

pub struct EnemyShipPlugin;

impl Plugin for EnemyShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_enemy_ships.in_set(UpdateSet::Movement));
    }
}
