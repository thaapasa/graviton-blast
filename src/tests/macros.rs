#[macro_export]
macro_rules! assert_vec_eq {
    ($lhs:expr, $x:expr, $y:expr) => {{
        assert!(
            float_cmp::approx_eq!(f32, $lhs.x, $x, epsilon = 0.00001),
            "Vec2.x mismatch: got {}, expected {} (lhs={:?})",
            $lhs.x,
            $x,
            $lhs
        );
        assert!(
            float_cmp::approx_eq!(f32, $lhs.y, $y, epsilon = 0.00001),
            "Vec2.y mismatch: got {}, expected {} (lhs={:?})",
            $lhs.y,
            $y,
            $lhs
        );
    }};
}
