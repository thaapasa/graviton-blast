use crate::assets::GameSprite;
use crate::core::components::PlayerShip;
use crate::core::health::{Health, MaxHealth};
use bevy::prelude::*;

/// [`GlobalZIndex`] used to render the ship status overlay.
pub const SHIP_STATUS_ZINDEX: i32 = i32::MAX - 20;

#[derive(Component, Debug)]
pub struct ShipStatus;

impl ShipStatus {
    const SCALE: f32 = GameSprite::ShipStatus.scale();
    const BAR_TOP: f32 = 47.0 * Self::SCALE;
    const BAR_HEIGHT: f32 = 136.0 * Self::SCALE;
    const BAR_LEFT: f32 = 210.0 * Self::SCALE;
    const BAR_WIDTH: f32 = 19.0 * Self::SCALE;
}

pub fn spawn_ship_status(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn((
            ShipStatus,
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
                top: Val::Px(ShipStatus::BAR_TOP),
                left: Val::Px(ShipStatus::BAR_LEFT),
                width: Val::Px(ShipStatus::BAR_WIDTH),
                height: Val::Px(ShipStatus::BAR_HEIGHT),
                ..Default::default()
            },
            ImageNode::solid_color(Color::linear_rgb(0.1, 0.9, 0.4)),
        ));
}

pub fn update_player_status(
    player_q: Query<(&Health, &MaxHealth), With<PlayerShip>>,
    query: Query<Entity, With<ShipStatus>>,
    children: Query<&Children>,
    mut nodes: Query<&mut Node>,
) {
    let (health, max_health) = player_q.single().unwrap();
    let status = query.single().unwrap();
    let children = children.get(status).unwrap();
    let child = children.first().unwrap();
    let mut node = nodes.get_mut(*child).unwrap();

    let amount = (health.value() / max_health.value()).clamp(0.0, 1.0);
    let bar_height = amount * ShipStatus::BAR_HEIGHT;

    node.top = Val::Px(ShipStatus::BAR_TOP + (ShipStatus::BAR_HEIGHT - bar_height));
    node.height = Val::Px(bar_height);
}
