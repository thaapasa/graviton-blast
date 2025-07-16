use crate::assets::GameSprite;
use bevy::prelude::*;

/// [`GlobalZIndex`] used to render the ship status overlay.
pub const SHIP_STATUS_ZINDEX: i32 = i32::MAX - 20;

#[derive(Component, Debug)]
pub struct ShipStatus;

pub fn spawn_ship_status(commands: &mut Commands, asset_server: &AssetServer) {
    let scale = GameSprite::ShipStatus.scale();
    commands
        .spawn((
            Name::new("ShipStatusNode"),
            Node {
                // We need to make sure the overlay doesn't affect the position of other UI nodes
                position_type: PositionType::Absolute,
                bottom: Val::Px(30.0),
                left: Val::Px(30.0),
                width: Val::Px(GameSprite::ShipStatus.scaled_size()),
                height: Val::Px(GameSprite::ShipStatus.scaled_size()),
                ..Default::default()
            },
            ImageNode::new(GameSprite::ShipStatus.load(asset_server)),
            GlobalZIndex(SHIP_STATUS_ZINDEX),
        ))
        .with_child((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(47.0 * scale),
                left: Val::Px(210.0 * scale),
                width: Val::Px(19.0 * scale),
                height: Val::Px(136.0 * scale),
                ..Default::default()
            },
            ImageNode::solid_color(Color::linear_rgb(0.1, 0.9, 0.4)),
        ));
}
