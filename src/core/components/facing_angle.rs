use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::ops::{Add, AddAssign, SubAssign};

use crate::core::components::Velocity;

// 2D and 3D scenes and cameras
// https://bevy-cheatbook.github.io/fundamentals/coords.html#coordinate-system
//
// Bevy uses a right-handed Y-up coordinate system for the game world. The coordinate
// system is the same for 3D and 2D, for consistency.
//
// It is easiest to explain in terms of 2D:
//
// - The X axis goes from left to right (+X points right).
// - The Y axis goes from bottom to top (+Y points up).
// - The Z axis goes from far to near (+Z points towards you, out of the screen).
// - For 2D, the origin (X=0.0; Y=0.0) is at the center of the screen by default.

/// Facing angle, in radians
#[derive(Component, Debug, Copy, Clone, Default, PartialEq)]
pub struct FacingAngle(pub f32);

/// Marker trait. Add this to components with ``FacingAngle`` and ``Velocity``
/// to make them automatically rotate according to their velocity.
#[derive(Component, Debug)]
pub struct AngleFollowsVelocity;

impl FacingAngle {
    /// Right = 0 degrees = 0 radians
    #[allow(dead_code)]
    pub const RIGHT: Self = FacingAngle(0.0);
    /// Up = 90 degrees = PI/2 radians
    #[allow(dead_code)]
    pub const UP: Self = FacingAngle(FRAC_PI_2);
    /// Left = 180 degrees = PI radians
    #[allow(dead_code)]
    pub const LEFT: Self = FacingAngle(PI);
    /// Down = 270 degrees = 3PI/2 radians
    #[allow(dead_code)]
    pub const DOWN: Self = FacingAngle(PI + FRAC_PI_2);

    #[inline]
    pub fn new(angle: f32) -> Self {
        Self(angle.rem_euclid(TAU))
    }

    #[allow(dead_code)]
    pub fn as_vec(&self, magnitude: f32) -> Vec2 {
        if magnitude.is_sign_negative() {
            -Vec2::from_angle(self.0) * -magnitude
        } else {
            Vec2::from_angle(self.0) * magnitude
        }
    }

    #[inline]
    pub fn to_velocity(self, magnitude: f32) -> Velocity {
        Velocity::new(self.as_vec(magnitude))
    }

    pub fn as_quat(&self) -> Quat {
        Quat::from_rotation_z(self.0)
    }

    /// Returns the direction that's opposite this direction.
    #[inline]
    pub fn flip(&self) -> Self {
        FacingAngle::new(self.0 + PI)
    }

    /// Returns the shortest angle difference between this angle and the given angle.
    /// Returns a value in the range ``[-PI, PI]`` radians.
    #[inline]
    pub fn angle_diff(&self, angle: f32) -> f32 {
        // Find angles in both rotation directions
        let (a1, a2) = (angle - self.0, angle - (self.0 + TAU));
        // Find Least angle diff
        let diff = if a1.abs() <= a2.abs() { a1 } else { a2 };
        // It might still be out of bounds, so shift to correct range
        if diff < -PI {
            diff + TAU
        } else if diff > PI {
            diff - TAU
        } else {
            diff
        }
    }

    pub fn rotate_towards(&self, target_angle: f32, rotation_amount: f32) -> Self {
        let angle_diff = self.angle_diff(target_angle);
        let amount = angle_diff.abs().min(rotation_amount);
        FacingAngle(self.0 + amount.copysign(angle_diff))
    }
}

impl From<f32> for FacingAngle {
    fn from(radians: f32) -> Self {
        Self(radians)
    }
}

impl Add<f32> for FacingAngle {
    type Output = FacingAngle;
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        FacingAngle((self.0 + rhs).rem_euclid(TAU))
    }
}

impl AddAssign<f32> for FacingAngle {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.0 = (self.0 + rhs).rem_euclid(TAU)
    }
}

impl SubAssign<f32> for FacingAngle {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.0 = (self.0 - rhs).rem_euclid(TAU)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_vec_eq;
    use crate::core::components::FacingAngle;
    use float_cmp::{approx_eq, assert_approx_eq};
    use rstest::rstest;
    use std::f32::consts::{FRAC_PI_4, SQRT_2};

    #[rstest]
    #[case(FacingAngle::RIGHT, 1.0, 1.0, 0.0)]
    #[case(FacingAngle::UP, 1.0, 0.0, 1.0)]
    #[case(FacingAngle::LEFT, 1.0, -1.0, 0.0)]
    #[case(FacingAngle::DOWN, 1.0, 0.0, -1.0)]
    #[case(FacingAngle::RIGHT, 1.5, 1.5, 0.0)]
    #[case(FacingAngle::UP, 1.5, 0.0, 1.5)]
    #[case(FacingAngle::LEFT, 1.5, -1.5, 0.0)]
    #[case(FacingAngle::DOWN, 1.5, 0.0, -1.5)]
    fn test_straight_angles(
        #[case] angle: FacingAngle,
        #[case] magnitude: f32,
        #[case] x: f32,
        #[case] y: f32,
    ) {
        assert_vec_eq!(&angle.as_vec(magnitude), x, y);
    }

    #[test]
    fn test_other_angles() {
        // 45 angles, "to northeast"
        // Double the magnitude to get nice, "round" numbers
        assert_vec_eq!(&FacingAngle(FRAC_PI_4).as_vec(2.0), SQRT_2, SQRT_2);
    }

    #[rstest]
    #[case(0.0, 20.0, 20.0)]
    #[case(0.0, 340.0, -20.0)]
    #[case(0.0, 180.0, 180.0)]
    #[case(45.0, 0.0, -45.0)]
    #[case(45.0, 5.0, -40.0)]
    #[case(45.0, 90.0, 45.0)]
    #[case(45.0, 180.0, 135.0)]
    #[case(45.0, 315.0, -90.0)]
    #[case(45.0, 270.0, -135.0)]
    #[case(270.0, 45.0, 135.0)]
    #[case(270.0, 90.0, -180.0)]
    #[case(270.0, 180.0, -90.0)]
    #[case(270.0, 350.0, 80.0)]
    #[case(300.0, 60.0, 120.0)]
    #[case(300.0, 200.0, -100.0)]
    fn test_angle_diff(#[case] angle: f32, #[case] target: f32, #[case] expected: f32) {
        assert_approx_eq!(
            f32,
            FacingAngle(angle.to_radians()).angle_diff(target.to_radians()),
            expected.to_radians(),
            epsilon = 0.001
        );
    }
}
