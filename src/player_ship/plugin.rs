use bevy::prelude::*;

use crate::player_ship::systems::{
    fire_player_weapons, update_player_movement,
};
use crate::player_ship::trail::{fade_particles, spawn_trail_particles};

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_player_movement,
                fire_player_weapons,
                spawn_trail_particles,
                fade_particles,
            ),
        );
    }
}
