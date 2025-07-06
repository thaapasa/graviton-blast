use bevy::app::{App, Plugin};
use bevy::prelude::*;

use crate::black_hole::systems::apply_gravity;
use crate::core::UpdateSet::Movement;

pub struct BlackHolePlugin;

impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_gravity.in_set(Movement));
    }
}
