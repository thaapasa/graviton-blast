use crate::core::components::{AngleFollowsVelocity, FacingAngle, Velocity};
use crate::projectile::components::ProjectileType;
use bevy::prelude::*;

pub fn spawn_projectile(
    commands: &mut Commands,
    asset_server: &AssetServer,
    projectile_type: ProjectileType,
    position: Vec2,
    velocity: Velocity,
    facing_angle: FacingAngle,
) {
    let sprite = projectile_type.sprite();
    commands.spawn((
        Name::new(format!("Projectile {projectile_type:?}")),
        projectile_type.create_projectile(),
        Sprite::from_image(sprite.load(asset_server)),
        velocity + facing_angle.to_velocity(projectile_type.speed()),
        facing_angle,
        AngleFollowsVelocity,
        sprite.initial_transform(position, facing_angle),
        projectile_type.mass(),
    ));
}
