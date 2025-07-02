use crate::background::systems::relocate_parallax_background;
use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, relocate_parallax_background);
    }
}
