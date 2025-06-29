#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    Clockwise,
    Anticlockwise,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Thrust(pub f32);

impl Thrust {
    #[allow(dead_code)]
    pub const ZERO: Thrust = Thrust(0.0);

    #[inline]
    pub fn has_thrust(&self) -> bool {
        self.0 != 0.0
    }
}
