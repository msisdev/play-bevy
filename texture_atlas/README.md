# /texture_atlas

- https://github.com/bevyengine/bevy/blob/main/examples/2d/texture_atlas.rs


## create_texture_atlas

Add all textures in the asset folder into `TextureAtlasBuilder`

Build `TextureAtlasBuilder` to get:
- `TextureAtlasLayout`
- `TextureAtlasSources`
- `Image`

Add texture `Image` into `Assets<Image>` to get
- texture `Handle<Image>`

Update image sampler if needed



## spawn_sprite_from_atlas
`TextureAtlasSources`(`Handle<TextureAtlasLayout>`, `Handle<Image>`) -> `TextureAtlas`
```rs
bevy_image::texture_atlas::TextureAtlasSources
pub fn handle(&self, layout: Handle<TextureAtlasLayout>, texture: impl Into<AssetId<Image>>) -> Option<TextureAtlas>
```

`Handle<Image>` + `TextureAtlas` -> `Sprite`
```rs
bevy_sprite::sprite::Sprite
pub fn from_atlas_image(image: Handle<Image>, atlas: TextureAtlas) -> Self
```
