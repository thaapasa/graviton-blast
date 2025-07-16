use crate::constants::FPS_REFRESH_INTERVAL;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use std::time::Duration;

/// [`GlobalZIndex`] used to render the fps overlay.
pub const FPS_OVERLAY_ZINDEX: i32 = i32::MAX;

#[derive(Component, Debug)]
pub struct FpsText;

pub fn spawn_fps_counter(commands: &mut Commands) {
    commands
        .spawn((
            Name::new("FpsCounterNode"),
            Node {
                // We need to make sure the overlay doesn't affect the position of other UI nodes
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                width: Val::Px(100.0),
                ..Default::default()
            },
            GlobalZIndex(FPS_OVERLAY_ZINDEX),
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("FPSCounter"),
                Text::new("FPS: "),
                FpsText,
                TextFont {
                    font: Handle::<Font>::default(),
                    font_size: 14.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.with_alpha(0.5)),
            ))
            .with_child((
                TextSpan::default(),
                TextFont {
                    font: Handle::<Font>::default(),
                    font_size: 14.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.with_alpha(0.5)),
            ));
        });
}

pub fn update_fps(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
    mut time_since_rerender: Local<Duration>,
) {
    *time_since_rerender += time.delta();
    if *time_since_rerender < FPS_REFRESH_INTERVAL {
        return;
    }
    *time_since_rerender = Duration::ZERO;
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        for text in &query {
            if let Some(value) = fps.smoothed() {
                *writer.text(text, 1) = format!("{value:.1}");
            }
        }
    }
}
