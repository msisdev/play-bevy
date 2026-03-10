use bevy::prelude::*;
use bevy::image::{ImagePlugin, ImageSamplerDescriptor};
use bevy::window::WindowResolution;
use sprite_char::player::load_player;

mod config {
    pub const WINDOW_TITLE: &str = "sprite_char";
    pub const WINDOW_PHYSICAL_WIDTH: u32 = 480;
    pub const WINDOW_PHYSICAL_HEIGHT: u32 = 480;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor::nearest(),
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: config::WINDOW_TITLE.to_string(),
                    resolution: WindowResolution::new(
                        config::WINDOW_PHYSICAL_WIDTH,
                        config::WINDOW_PHYSICAL_HEIGHT,
                    ),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                sprite_char::player::move_player,
            ),
        )
        .add_systems(
            Update,
            (
                sprite_char::animate::animate_sprite,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    load_player(&mut commands, &asset_server, &mut texture_atlas_layouts);
}
