mod entities;

use crate::entities::player;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(player::create_resource())
        .add_systems(Startup, setup);

    player::add_systems(&mut app);
    app.run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
