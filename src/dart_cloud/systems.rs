use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::core::components::{NextVelocity, SpatialTree, Velocity};
use crate::dart_cloud::components::Dart;

pub fn move_dart(
    mut query: Query<(Entity, &Transform, &mut NextVelocity), With<Dart>>,
    dart_query: Query<(&Transform, &Velocity), With<Dart>>,
    tree: Res<SpatialTree>,
) {
    for (dart, pos, mut next_velocity) in &mut query {
        let dart_pos = pos.translation.truncate();
        let mut nearest = Vec::new();
        for (_, neighbor) in tree.k_nearest_neighbour(dart_pos, 10) {
            let Some(entity) = neighbor else { continue };
            if entity != dart {
                if let Ok((t, v)) = dart_query.get(entity) {
                    let pos = t.translation.truncate();
                    nearest.push((pos, *v, pos - dart_pos));
                }
            }
        }
        if nearest.is_empty() {
            continue;
        }
        let avoid = BoidSystem::avoidance(&nearest);
        let align = BoidSystem::alignment(&nearest);
        let cohes = BoidSystem::cohesion(dart_pos, &nearest);
        let avoid_weight = BoidSystem::avoid_weight(&nearest);
        let other_weight = 1.0 - avoid_weight;
        let total = (avoid * avoid_weight) + (align * 2.0 + cohes) * other_weight / 3.0;
        *next_velocity = NextVelocity(total * BoidSystem::MAX_VELOCITY)
    }
}

/// Vec of (Position, Velocity, Delta)
type NearestBoids = [(Vec2, Velocity, Vec2)];
struct BoidSystem;

impl BoidSystem {
    const SEPARATION: f32 = 50.0;
    const SEPARATION_SQUARED: f32 = BoidSystem::SEPARATION * BoidSystem::SEPARATION;
    const MAX_VELOCITY: f32 = 400.0;

    /// Returns the weight of avoidance, in the range of `0..1`.
    fn avoid_weight(nearest: &NearestBoids) -> f32 {
        let Some(first) = nearest.first() else {
            return 0.0;
        };
        let min_dist_sq = first.2.length_squared();
        if min_dist_sq < Self::SEPARATION_SQUARED {
            (Self::SEPARATION_SQUARED - min_dist_sq) / Self::SEPARATION_SQUARED
        } else {
            0.0
        }
    }

    fn avoidance(nearest: &NearestBoids) -> Vec2 {
        let mut avoid = Vec2::ZERO;
        for (_, _, delta) in nearest {
            if delta.length_squared() < Self::SEPARATION_SQUARED {
                avoid += -delta;
            }
        }
        avoid.normalize_or_zero()
    }

    fn alignment(nearest: &NearestBoids) -> Vec2 {
        nearest
            .iter()
            .fold(Vec2::ZERO, |a, (_, vel, _)| a + **vel)
            .normalize_or_zero()
    }

    fn cohesion(current_pos: Vec2, nearest: &NearestBoids) -> Vec2 {
        let center =
            nearest.iter().fold(Vec2::ZERO, |a, (pos, _, _)| a + pos) / nearest.len() as f32;
        (center - current_pos).normalize_or_zero()
    }
}
