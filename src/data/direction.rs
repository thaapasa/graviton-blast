use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::ops::{AddAssign, SubAssign};

/// Direction, in radians
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Direction(pub f32);

impl Direction {
    /// Right = 0 degrees = 0 radians
    #[allow(dead_code)]
    pub const RIGHT: Self = Direction(0.0);
    /// Up = 90 degrees = PI/2 radians
    #[allow(dead_code)]
    pub const UP: Self = Direction(FRAC_PI_2);
    /// Left = 180 degrees = PI radians
    #[allow(dead_code)]
    pub const LEFT: Self = Direction(PI);
    /// Down = 270 degrees = 3PI/2 radians
    #[allow(dead_code)]
    pub const DOWN: Self = Direction(PI + FRAC_PI_2);

    pub fn as_vec(&self, magnitude: f32) -> Vec2 {
        Vec2::from_angle(self.0) * magnitude
    }

    pub fn as_quat(&self) -> Quat {
        Quat::from_rotation_z(self.0)
    }
}

impl From<f32> for Direction {
    fn from(radians: f32) -> Self {
        Self(radians)
    }
}

impl AddAssign for Direction {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0).rem_euclid(TAU)
    }
}

impl SubAssign for Direction {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = (self.0 - rhs.0).rem_euclid(TAU)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_vec_eq;
    use crate::data::Direction;
    use float_cmp::approx_eq;
    use std::f32::consts::{FRAC_PI_4, SQRT_2};

    #[test]
    fn test_straight_angles() {
        assert_vec_eq!(&Direction::RIGHT.as_vec(1.0), 1.0, 0.0);
        assert_vec_eq!(&Direction::UP.as_vec(1.0), 0.0, 1.0);
        assert_vec_eq!(&Direction::LEFT.as_vec(1.0), -1.0, 0.0);
        assert_vec_eq!(&Direction::DOWN.as_vec(1.0), 0.0, -1.0);

        assert_vec_eq!(&Direction::RIGHT.as_vec(1.5), 1.5, 0.0);
        assert_vec_eq!(&Direction::UP.as_vec(1.5), 0.0, 1.5);
        assert_vec_eq!(&Direction::LEFT.as_vec(1.5), -1.5, 0.0);
        assert_vec_eq!(&Direction::DOWN.as_vec(1.5), 0.0, -1.5);
    }

    #[test]
    fn test_other_angles() {
        // 45 angles, "to northeast"
        // Double the magnitude to get nice, "round" numbers
        assert_vec_eq!(&Direction(FRAC_PI_4).as_vec(2.0), SQRT_2, SQRT_2);
    }
}
