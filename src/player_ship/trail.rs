use bevy::math::Vec3;
use bevy::prelude::*;

use crate::assets::{DrawingOrder, GameSprite};
use crate::core::components::{FacingAngle, PlayerShip, Velocity};
use crate::core::resources::PlayerActions;
use crate::player_ship::components::TrailParticle;

const PARTICLE_LIFETIME_SECS: f32 = 1.0;

pub fn spawn_trail_particles(
    mut commands: Commands,
    query: Query<(&Transform, &FacingAngle, &Velocity), With<PlayerShip>>,
    asset_server: Res<AssetServer>,
    actions: Res<PlayerActions>,
) {
    match actions.thrust {
        Some(true) => (),
        _ => return,
    }
    let (transform, angle, velocity) = query.single().unwrap();
    let pos = transform.translation;
    let trail_angle = angle.flip();

    commands.spawn((
        Sprite::from_image(GameSprite::ExhaustRing.load(&asset_server)),
        Transform::from_translation(DrawingOrder::EngineTrail.relocate_to_z(pos))
            .with_scale(Vec3::splat(GameSprite::ExhaustRing.scale())),
        TrailParticle {
            lifetime: PARTICLE_LIFETIME_SECS,
        },
        trail_angle,
        trail_angle.to_velocity(200.0) + *velocity,
    ));
}

pub fn fade_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut TrailParticle, &mut Sprite)>,
) {
    for (entity, mut transform, mut particle, mut sprite) in &mut query {
        particle.lifetime -= time.delta_secs();
        sprite
            .color
            .set_alpha(0.8 * particle.lifetime / PARTICLE_LIFETIME_SECS);
        transform.scale *= 1.0 + time.delta_secs() * 2.0;

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
