use bevy::input::ButtonInput;
use bevy::prelude::*;

use crate::core::resources::PlayerActions;
use crate::core::Rotation;

pub fn map_input_to_player_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut actions: ResMut<PlayerActions>,
) {
    let rotate_left = keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA);
    let rotate_right = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);
    match (rotate_left, rotate_right) {
        (true, false) => actions.rotate = Some(Rotation::Anticlockwise),
        (false, true) => actions.rotate = Some(Rotation::Clockwise),
        _ => actions.rotate = None,
    }
    let fwd = keyboard.pressed(KeyCode::ArrowUp);
    let bwd = keyboard.pressed(KeyCode::ArrowDown);
    match (fwd, bwd) {
        (true, false) => actions.thrust = Some(true),
        (false, true) => actions.thrust = Some(false),
        _ => actions.thrust = None,
    }
    actions.fire = keyboard.just_pressed(KeyCode::Space);
    actions.quit = keyboard.just_pressed(KeyCode::Escape);
}

pub fn quit_if_requested(actions: Res<PlayerActions>, mut exit: EventWriter<AppExit>) {
    if actions.quit {
        exit.write(AppExit::Success);
    }
}
