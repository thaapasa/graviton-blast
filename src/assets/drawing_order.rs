use bevy::prelude::*;

/// Determines the entities that are drawn in the game and their respective z-orders.
#[derive(Copy, Clone, Debug)]
pub enum DrawingOrder {
    #[allow(dead_code)]
    BlackHole,
    /// The engine exhaust trails shown behind ships
    EngineTrail,
    #[allow(dead_code)]
    EnemyShip,
    PlayerShip,
}

impl DrawingOrder {
    /// Returns the z-coordinate for this entity, to be used for making sure entities are drawn
    /// in the correct order.
    #[inline]
    pub fn z_order(self) -> f32 {
        match self {
            Self::BlackHole => 0.0,
            Self::EngineTrail => 5.0,
            Self::EnemyShip => 9.0,
            Self::PlayerShip => 10.0,
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
