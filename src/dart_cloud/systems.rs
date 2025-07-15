use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::core::components::{FacingAngle, PlayerShip, SpatialTree, Velocity};
use crate::dart_cloud::components::{Dart, DartCloud};
use crate::utils::Vec2Ext;

pub fn move_dart(
    player_q: Query<&Transform, With<PlayerShip>>,
    mut query: Query<(Entity, &Transform, &mut FacingAngle), With<Dart>>,
    dart_query: Query<(&Transform, &Velocity), With<Dart>>,
    tree: Res<SpatialTree>,
) {
    let player_pos = player_q.single().unwrap().translation.truncate();
    for (dart, pos, mut facing_angle) in &mut query {
        let dart_pos = pos.translation.truncate();
        let mut nearest = Vec::new();
        for (_, neighbor) in tree.k_nearest_neighbour(dart_pos, DartCloud::NEIGHBORS_TO_CALC) {
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
        let avoid = DartCloud::avoidance(&nearest);
        let align = DartCloud::alignment(&nearest);
        let cohes = DartCloud::cohesion(dart_pos, &nearest);
        let avoid_weight = DartCloud::avoid_weight(&nearest);

        let to_player = player_pos - dart_pos;
        let to_player_len_sq = to_player.length_squared();

        let total = if to_player_len_sq < DartCloud::TARGET_PLAYER_DISTANCE_SQUARED {
            let weight = ((DartCloud::TARGET_PLAYER_DISTANCE_SQUARED - to_player_len_sq)
                / DartCloud::TARGET_PLAYER_DISTANCE_SQUARED)
                .clamp(0.0, DartCloud::MAX_PLAYER_VEC_WEIGHT);
            cohes.add_with_weight(&to_player, weight)
        } else {
            cohes
        }
        .add_with_weight(&align, DartCloud::ALIGNMENT_WEIGHT)
        .add_with_weight(&avoid, avoid_weight);

        *facing_angle = FacingAngle::new(total.to_angle());
    }
}

type NearestBoid = (Vec2, Velocity, Vec2);
/// Vec of (Position, Velocity, Delta)
type NearestBoids = [NearestBoid];

impl DartCloud {
    /// Returns the weight of avoidance, in the range of `0..1`.
    fn avoid_weight(nearest: &NearestBoids) -> f32 {
        let Some(first) = nearest.first() else {
            return 0.0;
        };
        let min_dist_sq = first.2.length_squared();
        if min_dist_sq < DartCloud::SEPARATION_SQUARED {
            (DartCloud::SEPARATION - min_dist_sq.sqrt()) / DartCloud::SEPARATION
        } else {
            0.0
        }
    }

    /// Vector that tries to avoid nearest neighbors
    fn avoidance(nearest: &NearestBoids) -> Vec2 {
        let mut avoid = Vec2::ZERO;
        for (_, _, delta) in nearest {
            if delta.length_squared() < DartCloud::SEPARATION_SQUARED {
                avoid += -delta;
            }
        }
        avoid.normalize_or_zero()
    }

    /// Average velocity of nearest neighbors
    fn alignment(nearest: &NearestBoids) -> Vec2 {
        nearest
            .iter()
            .fold(Vec2::ZERO, |a, (_, vel, _)| a + **vel)
            .normalize_or_zero()
    }

    /// Target the center of nearest neighbors
    fn cohesion(current_pos: Vec2, nearest: &NearestBoids) -> Vec2 {
        let center =
            nearest.iter().fold(Vec2::ZERO, |a, (pos, _, _)| a + pos) / nearest.len() as f32;
        (center - current_pos).normalize_or_zero()
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use float_cmp::assert_approx_eq;
    use std::f32::consts::FRAC_1_SQRT_2;

    use crate::assert_vec_eq;
    use crate::core::components::Velocity;
    use crate::dart_cloud::components::DartCloud;

    #[test]
    fn test_avoid_weight() {
        use fixtures::{FAR1, NEAR1, NEAR2, neighbor};
        assert_approx_eq!(f32, DartCloud::avoid_weight(&[neighbor(FAR1)]), 0.0);
        let d1 = DartCloud::avoid_weight(&[neighbor(NEAR1)]);
        let d2 = DartCloud::avoid_weight(&[neighbor(NEAR2)]);
        assert!(d1 < 1.0);
        assert!(d1 > 0.5);
        assert!(d2 < 1.0);
        assert!(d2 > 0.5);
        assert!(d2 < d1);
        assert_approx_eq!(
            f32,
            DartCloud::avoid_weight(&[neighbor(NEAR1), neighbor(NEAR2), neighbor(FAR1)]),
            d1
        );
    }

    #[test]
    fn test_avoidance() {
        use fixtures::{NEAR1, NEAR2, neighbor};
        assert_vec_eq!(
            DartCloud::avoidance(&[neighbor(NEAR1)]),
            -0.74740934,
            0.66436386
        );
        assert_vec_eq!(
            DartCloud::avoidance(&[neighbor(NEAR2)]),
            0.6459423,
            0.7633863
        );
        assert_vec_eq!(
            DartCloud::avoidance(&[neighbor(NEAR1), neighbor(NEAR2)]),
            0.0948091,
            0.99549556
        );
    }

    #[test]
    fn test_alignment() {
        use fixtures::{NEAR1, NEAR2, neighbor_v};
        let v1 = Velocity::new(Vec2::new(10.0, 0.0).normalize());
        let v2 = Velocity::new(Vec2::new(0.0, 2.0).normalize());
        assert_vec_eq!(DartCloud::alignment(&[neighbor_v(NEAR1, v1)]), 1.0, 0.0);
        assert_vec_eq!(DartCloud::alignment(&[neighbor_v(NEAR2, v2)]), 0.0, 1.0);
        assert_vec_eq!(
            DartCloud::alignment(&[neighbor_v(NEAR1, v1), neighbor_v(NEAR2, v2)]),
            FRAC_1_SQRT_2,
            FRAC_1_SQRT_2
        );
    }

    #[test]
    fn test_cohesion() {
        use fixtures::{ME, NEAR1, NEAR2, neighbor};
        assert_vec_eq!(
            DartCloud::cohesion(ME, &[neighbor(NEAR1)]),
            0.74740934,
            -0.66436386
        );
        assert_vec_eq!(
            DartCloud::cohesion(ME, &[neighbor(NEAR2)]),
            -0.6459423,
            -0.7633863
        );
        assert_vec_eq!(
            DartCloud::cohesion(ME, &[neighbor(NEAR1), neighbor(NEAR2)]),
            -0.0948091,
            -0.99549556
        );
    }

    mod fixtures {
        use bevy::prelude::Vec2;

        use crate::core::components::Velocity;
        use crate::dart_cloud::systems::NearestBoid;

        pub const ME: Vec2 = Vec2::new(20.0, 20.0);
        pub const NEAR1: Vec2 = Vec2::new(29.0, 12.0);
        pub const NEAR2: Vec2 = Vec2::new(9.0, 7.0);
        pub const FAR1: Vec2 = Vec2::new(-450.0, 250.0);

        pub fn neighbor(n: Vec2) -> NearestBoid {
            (n, Velocity::ZERO, n - ME)
        }
        pub fn neighbor_v(n: Vec2, velocity: Velocity) -> NearestBoid {
            (n, velocity, n - ME)
        }
    }
}
