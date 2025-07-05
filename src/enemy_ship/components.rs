use crate::assets::GameSprite;
use crate::constants::{ENEMY_SHIP_1_MASS, ENEMY_SHIP_1_MAX_VELOCITY};
use crate::core::components::{Mass, MaxVelocity};
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct EnemyShip;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum EnemyShipType {
    Enemy1,
}

impl EnemyShipType {
    pub fn sprite(&self) -> GameSprite {
        match self {
            Self::Enemy1 => GameSprite::EnemyShip1,
        }
    }

    pub fn mass(&self) -> Mass {
        match self {
            Self::Enemy1 => ENEMY_SHIP_1_MASS,
        }
    }

    pub fn max_velocity(&self) -> MaxVelocity {
        match self {
            Self::Enemy1 => ENEMY_SHIP_1_MAX_VELOCITY,
        }
    }
}
