use bevy::prelude::*;

/// Determines the entities that are drawn in the game and their respective z-orders.
#[derive(Copy, Clone, Debug)]
pub enum DrawingOrder {
    StarsBg,
    StarsFg,
    BlackHole,
    /// The engine exhaust trails shown behind ships
    EngineTrail,
    Projectile,
    Dart,
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
            Self::StarsBg => 0.0,
            Self::StarsFg => 1.0,
            Self::BlackHole => 5.0,
            Self::EngineTrail => 10.0,
            Self::Projectile => 30.0,
            Self::Dart => 35.0,
            Self::EnemyShip => 40.0,
            Self::PlayerShip => 50.0,
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
