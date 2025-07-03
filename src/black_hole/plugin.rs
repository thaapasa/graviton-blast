use crate::black_hole::systems::apply_gravity;
use bevy::app::{App, Plugin, Update};

pub struct BlackHolePlugin;

impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}
