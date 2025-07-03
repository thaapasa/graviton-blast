use bevy::prelude::*;

use crate::background::systems::{on_resize_window, relocate_parallax_background};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_resize_window);
        app.add_systems(Update, relocate_parallax_background);
    }
}
