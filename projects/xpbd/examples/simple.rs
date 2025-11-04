use bevy::prelude::*;
use xpbd::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(XPBDPlugin)
        .run();
}