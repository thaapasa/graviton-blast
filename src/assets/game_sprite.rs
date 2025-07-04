use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

use crate::assets::DrawingOrder;
use crate::core::components::FacingAngle;

/// A game sprite. All the sprites are rectangular and the sizes are
/// multiples of two.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameSprite {
    #[allow(dead_code)]
    StarsSparse,
    #[allow(dead_code)]
    StarsLarge,
    #[allow(dead_code)]
    BlackHole,
    ShotBlueBlaster,
    ExhaustRing,
    PlayerShip,
}

impl GameSprite {
    /// Returns the rotation (in radians) required to apply to the texture to make
    /// it correspond with the game's natural (mathematical) rotation, where 0 points
    /// to positive X.
    ///
    /// Add this to value to any object's in-game rotation to draw it correctly to the screen.
    pub const INHERENT_TEXTURE_ROTATION: f32 = -FRAC_PI_2;

    #[inline]
    pub fn filename(&self) -> &str {
        match self {
            Self::StarsSparse => "stars-sparse",
            Self::StarsLarge => "stars-large",
            Self::BlackHole => "black-hole",
            Self::ShotBlueBlaster => "shot-blue-blaster",
            Self::ExhaustRing => "exhaust",
            Self::PlayerShip => "player-ship",
        }
    }

    /// Returns the scale this sprite should be drawn to on the game screen by default.
    #[inline]
    pub fn scale(&self) -> f32 {
        match self {
            Self::StarsSparse => 1.0,
            Self::StarsLarge => 1.0,
            Self::ShotBlueBlaster => 0.2,
            Self::PlayerShip => 0.2,
            Self::ExhaustRing => 0.05,
            Self::BlackHole => 0.3,
        }
    }

    /// Returns the size of this square sprite (width and/or height, since they are both the same),
    /// in pixels.
    #[inline]
    pub fn size(&self) -> usize {
        match self {
            Self::ExhaustRing | Self::ShotBlueBlaster => 128,
            Self::PlayerShip | Self::BlackHole => 256,
            Self::StarsSparse | Self::StarsLarge => 1024,
        }
    }

    #[inline]
    pub fn scaled_size(&self) -> f32 {
        (self.size() as f32) * self.scale()
    }

    #[inline]
    fn path(&self) -> String {
        format!("sprites/{}.png", self.filename())
    }

    /// Returns the rotation (quaternion) to be applied to this texture
    /// to show it facing the given ``FacingAngle``.
    pub fn sprite_rotation(facing_angle: FacingAngle) -> Quat {
        (facing_angle + Self::INHERENT_TEXTURE_ROTATION).as_quat()
    }

    pub fn initial_transform(&self, starting_pos: Vec2) -> Transform {
        Transform::from_translation(DrawingOrder::from(self).to_vec_3d(starting_pos))
            .with_scale(Vec3::splat(self.scale()))
    }

    pub fn load(&self, asset_server: &AssetServer) -> Handle<Image> {
        asset_server.load(self.path())
    }
}

impl From<&GameSprite> for DrawingOrder {
    fn from(value: &GameSprite) -> Self {
        match value {
            GameSprite::StarsSparse => DrawingOrder::StarsBg,
            GameSprite::StarsLarge => DrawingOrder::StarsFg,
            GameSprite::ShotBlueBlaster => DrawingOrder::Projectile,
            GameSprite::PlayerShip => DrawingOrder::PlayerShip,
            GameSprite::ExhaustRing => DrawingOrder::EngineTrail,
            GameSprite::BlackHole => DrawingOrder::BlackHole,
        }
    }
}
