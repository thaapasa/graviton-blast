use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum DrawingOrder {
    #[allow(dead_code)]
    BlackHole,
    Contrail,
    PlayerShip,
}

impl DrawingOrder {
    /// Returns the z-coordinate for this entity, to be used for making sure entities are drawn
    /// in the correct order.
    #[inline]
    pub fn z_order(self) -> f32 {
        match self {
            Self::BlackHole => 0.0,
            Self::Contrail => 0.5,
            Self::PlayerShip => 1.0,
        }
    }

    /// Returns the correct 3d position in the game to draw this entity in the given 2d position.
    #[inline]
    pub fn to_vec_3d(self, pos: Vec2) -> Vec3 {
        pos.extend(self.z_order())
    }

    pub fn relocate_to_z(self, pos: Vec3) -> Vec3 {
        pos.truncate().extend(self.z_order())
    }
}
