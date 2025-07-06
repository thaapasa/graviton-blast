use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::dart_cloud::systems::move_dart;

pub struct DartCloudPlugin;

impl Plugin for DartCloudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, move_dart.in_set(UpdateSet::Planning));
    }
}
