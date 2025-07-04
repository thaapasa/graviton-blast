use crate::assets::GameSprite;
use crate::core::components::Mass;
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
            Self::Enemy1 => Mass::kg(1500.0),
        }
    }
}
