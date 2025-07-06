use bevy::prelude::*;

use crate::background::systems::{on_resize_window, relocate_parallax_background};
use crate::core::UpdateSet::{Finalize, Planning};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_resize_window.in_set(Planning),
                relocate_parallax_background.in_set(Finalize),
            ),
        );
    }
}
