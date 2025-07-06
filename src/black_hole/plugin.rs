use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoScheduleConfigs;

use crate::black_hole::systems::apply_gravity;
use crate::core::UpdateSet::Movement;

pub struct BlackHolePlugin;

impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity.in_set(Movement));
    }
}
