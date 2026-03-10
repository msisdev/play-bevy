use bevy::prelude::*;

use super::*;

pub fn load_player(
	commands: &mut Commands,
	asset_server: &AssetServer,
	texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
	let texture: Handle<Image> = asset_server.load(
		config::asset::DEFAULT,
	);
	
	let sprite = Sprite::from_atlas_image(
		texture,
		TextureAtlas {
			layout: texture_atlas_layouts.add(config::asset::default_layout()),
			index: config::animation_indice::DOWN_IDLE.0,
		},
	);

	commands.spawn(component::PlayerBundle {
		player: component::Player,
		transform: Transform::from_scale(Vec3::splat(2.0)),
		sprite,
		animation_indices: config::animation_indice::DOWN_IDLE,
		animation_timer: config::animation_timer::idle(),
	});
}
