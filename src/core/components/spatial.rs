use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree2;

#[derive(Component, Default)]
pub struct SpatialTracked;

pub type SpatialTree = KDTree2<SpatialTracked>;
