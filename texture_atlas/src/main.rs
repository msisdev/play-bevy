//! In this example we generate four texture atlases (sprite sheets) from a folder containing
//! individual sprites.
//!
//! The texture atlases are generated with different padding and sampling to demonstrate the
//! effect of these settings, and how bleeding issues can be resolved by padding the sprites.
//!
//! Only one padded and one unpadded texture atlas are rendered to the screen.
//! An upscaled sprite from each of the four atlases are rendered to the screen.

use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};
use texture_atlas::{defs, handler};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // fallback to nearest sampling
        .init_state::<defs::AppState>()
        .add_systems(OnEnter(defs::AppState::Setup), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(defs::AppState::Setup)))
        .add_systems(OnEnter(defs::AppState::Finished), setup)
        .run();
}

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load multiple, individual sprites from a folder
    commands.insert_resource(defs::RpgSpriteFolder(asset_server.load_folder("textures/rpg")));
}

fn check_textures(
    mut next_state: ResMut<NextState<defs::AppState>>,
    rpg_sprite_folder: Res<defs::RpgSpriteFolder>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
            next_state.set(defs::AppState::Finished);
        }
    }
}

fn setup(
    mut commands: Commands,
    rpg_sprite_handles: Res<defs::RpgSpriteFolder>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();

    // Create texture atlases with different padding and sampling

    let (
        texture_atlas_nearest,
        nearest_sources,
        nearest_texture,
    ) = handler::create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);

    commands.spawn(Camera2d);
    
    // Get handle to a sprite to render
    let vendor_handle: Handle<Image> = asset_server
        .get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png")
        .unwrap();

    let base_y = 80.0; // y position of the sprites

    // Render a sprite from the texture_atlas
    handler::spawn_sprite_from_atlas(
        &mut commands,
        (-150.0, base_y, 0.0),
        nearest_texture,
        nearest_sources,
        atlas_nearest_handle,
        &vendor_handle,
    );
}
