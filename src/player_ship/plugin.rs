use crate::core::UpdateSet;
use crate::player_ship::systems::{fire_player_weapons, update_player_movement};
use crate::player_ship::trail::{fade_particles, spawn_trail_particles};
use bevy::prelude::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                fire_player_weapons.in_set(UpdateSet::PlayerAction),
                update_player_movement.in_set(UpdateSet::Movement),
                spawn_trail_particles.in_set(UpdateSet::PostMovement),
            ),
        );
        app.add_systems(Update, (fade_particles.in_set(UpdateSet::Finalize),));
    }
}
