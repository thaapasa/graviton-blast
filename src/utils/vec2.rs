use bevy::prelude::*;

pub trait Vec2Ext {
    fn rem_euclid_scalar(&self, rhs: f32) -> Vec2;

    /// Clamps the length of `self` to `max_length_squared` if it exceeds it,
    /// using only squared operations.
    fn clamp_max_length_squared(self, max_length_squared: f32) -> Vec2;
}

impl Vec2Ext for Vec2 {
    #[inline]
    fn rem_euclid_scalar(&self, rhs: f32) -> Vec2 {
        Vec2::new(self.x.rem_euclid(rhs), self.y.rem_euclid(rhs))
    }

    #[inline]
    fn clamp_max_length_squared(self, max_length_squared: f32) -> Vec2 {
        let len_sq = self.length_squared();
        if len_sq > max_length_squared {
            // Compute the scale factor without taking sqrt(len_sq)
            let scale = max_length_squared / len_sq;
            self * scale.sqrt()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use rstest::rstest;
    use std::f32::consts::FRAC_1_SQRT_2;

    use crate::utils::Vec2Ext;

    #[rstest]
    #[case(Vec2::new(0.0, 360.0), 360.0, Vec2::new(0.0, 0.0))]
    #[case(Vec2::new(270.0, 90.0), 360.0, Vec2::new(270.0, 90.0))]
    #[case(Vec2::new(-1.0, 361.0), 360.0, Vec2::new(359.0, 1.0))]
    fn test_rem_euclid_scalar(#[case] vec: Vec2, #[case] rem: f32, #[case] expected: Vec2) {
        assert_eq!(vec.rem_euclid_scalar(rem), expected);
    }

    #[rstest]
    #[case(Vec2::new(10.0, 5.0), 20.0, Vec2::new(10.0, 5.0))]
    #[case(Vec2::new(10.0, 0.0), 10.0, Vec2::new(10.0, 0.0))]
    #[case(Vec2::new(15.0, 0.0), 10.0, Vec2::new(10.0, 0.0))]
    #[case(Vec2::new(0.0, 15.0), 10.0, Vec2::new(0.0, 10.0))]
    #[case(Vec2::new(10.0, 10.0), 10.0, Vec2::new(10.0 * FRAC_1_SQRT_2, 10.0 * FRAC_1_SQRT_2))]
    fn test_clamp_max_length_squared(
        #[case] vec: Vec2,
        #[case] max_length: f32,
        #[case] expected: Vec2,
    ) {
        assert_eq!(
            vec.clamp_max_length_squared(max_length * max_length),
            expected
        );
    }
}
