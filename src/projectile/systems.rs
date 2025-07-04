use crate::projectile::components::Projectile;
use bevy::prelude::*;

pub fn age_projectiles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile), With<Projectile>>,
) {
    for (entity, mut projectile) in &mut query {
        projectile.age(time.delta_secs());
        if projectile.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}
