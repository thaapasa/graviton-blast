use crate::entity::player::ship::Ship;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct TrailParticle {
    lifetime: f32,
}

pub fn spawn_trail_particles(mut commands: Commands, query: Query<&Transform, With<Ship>>) {
    let pos = query.single().unwrap().translation;

    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 0.5, 0.2, 0.5),
            ..default()
        },
        Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
        TrailParticle { lifetime: 2.0 },
    ));
}

pub fn fade_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailParticle, &mut Sprite)>,
) {
    for (entity, mut particle, mut sprite) in &mut query {
        particle.lifetime -= time.delta_secs();
        sprite.color.set_alpha(particle.lifetime / 2.0);

        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
