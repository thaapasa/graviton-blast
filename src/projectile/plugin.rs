use crate::projectile::systems::age_projectiles;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, age_projectiles);
    }
}
