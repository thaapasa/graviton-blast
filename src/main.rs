mod entity;
mod level;

use crate::entity::player;
use crate::level::level1;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(player::create_resource())
        .add_systems(Startup, setup);

    // Start with level 1
    level1::add_systems(&mut app);
    app.run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
