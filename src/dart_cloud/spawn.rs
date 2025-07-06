use bevy::prelude::*;

use crate::assets::GameSprite;
use crate::core::components::{
    AngleFollowsVelocity, FacingAngle, NextVelocity, SpatialTracked,
};
use crate::core::SpawnInfo;
use crate::dart_cloud::components::Dart;
use crate::utils::Vec2Ext;

pub fn spawn_dart_cloud(
    commands: &mut Commands,
    asset_server: &AssetServer,
    spawn_info: SpawnInfo,
    cloud_size: usize,
) {
    let texture = GameSprite::Dart.load(asset_server);
    for n in 0..cloud_size {
        let spiral_pos = Vec2::spiral_spread(n);
        let pos = spawn_info.as_location() + spiral_pos * 25.0;
        let angle = if spiral_pos == Vec2::ZERO {
            FacingAngle::UP
        } else {
            FacingAngle::from(spiral_pos.to_angle())
        };
        commands.spawn((
            Dart,
            Sprite::from_image(texture.clone()),
            GameSprite::Dart.initial_transform(pos, angle),
            angle.to_velocity(200.0),
            NextVelocity::default(),
            angle,
            SpatialTracked,
            AngleFollowsVelocity,
        ));
    }
}
