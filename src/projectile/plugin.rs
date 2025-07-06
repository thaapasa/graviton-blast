use crate::core::UpdateSet;
use crate::projectile::systems::age_projectiles;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, age_projectiles.in_set(UpdateSet::Finalize));
    }
}
