use bevy::prelude::*;

use crate::assets::GameSprite;
use crate::constants::PROJECTILE_BLASTER_MASS;
use crate::core::components::Mass;

pub enum ProjectileType {
    /// Basic player ship blaster fire (blue electric bolt)
    PlayerBlaster,
}

#[derive(Component, Debug)]
pub struct Projectile {
    lifetime: f32,
}

impl Projectile {
    #[inline]
    pub fn age(&mut self, delta_secs: f32) {
        self.lifetime -= delta_secs;
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }
}

impl ProjectileType {
    pub fn create_projectile(&self) -> Projectile {
        Projectile {
            lifetime: self.lifetime(),
        }
    }

    pub fn lifetime(&self) -> f32 {
        match self {
            Self::PlayerBlaster => 2.0,
        }
    }

    pub fn sprite(&self) -> GameSprite {
        match self {
            Self::PlayerBlaster => GameSprite::ShotBlueBlaster,
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            Self::PlayerBlaster => 700.0,
        }
    }

    pub fn mass(&self) -> Mass {
        match self {
            Self::PlayerBlaster => PROJECTILE_BLASTER_MASS,
        }
    }
}
