use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

/// Create a texture atlas with the given padding and sampling settings
/// from the individual sprites in the given folder.
pub fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    texture_atlas_builder.padding(padding.unwrap_or_default());

    // Add textures in the folder
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (
        texture_atlas_layout,
        texture_atlas_sources,
        texture,
    ) = texture_atlas_builder.build().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture_atlas_sources, texture)
}

/// Create and spawn a sprite from a texture atlas
pub fn spawn_sprite_from_atlas(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    atlas_texture: Handle<Image>,
    atlas_sources: TextureAtlasSources,
    atlas_handle: Handle<TextureAtlasLayout>,
    vendor_handle: &Handle<Image>,
) {
    commands.spawn((
        Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            scale: Vec3::splat(3.0),
            ..default()
        },
        Sprite::from_atlas_image(
            atlas_texture,
            atlas_sources.handle(atlas_handle, vendor_handle).unwrap(),
        ),
    ));
}

/// Create and spawn a label (text)
pub fn spawn_label(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    text: &str,
    text_style: TextFont,
) {
    commands.spawn((
        Text2d::new(text),
        text_style,
        TextLayout::new_with_justify(Justify::Center),
        Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            ..default()
        },
    ));
}
