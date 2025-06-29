mod data;
mod entity;
mod level;

mod images;
#[cfg(test)]
pub mod tests;

use crate::entity::player;
use crate::level::level1;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    player::insert_resources(&mut app);
    app.add_systems(Startup, setup);

    // Start with level 1
    level1::add_systems(&mut app);
    app.run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
