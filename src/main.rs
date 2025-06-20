use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Movement {
            direction: Vec2::new(1.0, 0.7).normalize() * 150.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_ball, spawn_trail_particles, fade_particles))
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]

struct TrailParticle {
    lifetime: f32,
}

#[derive(Resource)]
struct Movement {
    direction: Vec2,
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(2.0)), // Adjust size as needed
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        Ball,
    ));
}

fn move_ball(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ball>>,
    mut movement: ResMut<Movement>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let mut transform = query.single_mut().unwrap();
    let delta = movement.direction * time.delta_secs();
    transform.translation += delta.extend(0.0);

    // Bounce off window edges
    let bounds = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let pos = transform.translation;

    if pos.x.abs() > bounds.x {
        movement.direction.x *= -1.0;
    }
    if pos.y.abs() > bounds.y {
        movement.direction.y *= -1.0;
    }
}

fn spawn_trail_particles(mut commands: Commands, query: Query<&Transform, With<Ball>>) {
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

fn fade_particles(
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
