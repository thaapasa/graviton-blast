use bevy::prelude::*;

pub trait Vec2Ext {
    fn rem_euclid_scalar(&self, rhs: f32) -> Vec2;
}

impl Vec2Ext for Vec2 {
    #[inline]
    fn rem_euclid_scalar(&self, rhs: f32) -> Vec2 {
        Vec2::new(self.x.rem_euclid(rhs), self.y.rem_euclid(rhs))
    }
}
