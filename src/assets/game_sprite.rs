use crate::assets::DrawingOrder;
use crate::core::components::FacingAngle;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameSprite {
    PlayerShip,
    ExhaustRing,
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
            Self::PlayerShip => "player-ship",
            Self::ExhaustRing => "exhaust",
        }
    }

    #[inline]
    pub fn scale(&self) -> f32 {
        match self {
            Self::PlayerShip => 0.2,
            Self::ExhaustRing => 0.05,
        }
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
            GameSprite::PlayerShip => DrawingOrder::PlayerShip,
            GameSprite::ExhaustRing => DrawingOrder::EngineTrail,
        }
    }
}
