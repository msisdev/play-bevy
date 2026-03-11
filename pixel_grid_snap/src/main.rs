//! Shows how to create graphics that snap to the pixel grid by rendering to a texture in 2D

use bevy::prelude::*;
use pixel_grid_snap::system::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup_camera, setup_sprite, setup_mesh))
        .add_systems(Update, (rotate, fit_canvas))
        .run();
}
