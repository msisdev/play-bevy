use bevy::prelude::*;
use crate::config;
use crate::player::internal;

fn load_sprite(
	asset_server: &AssetServer,
	texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Sprite {
	let texture: Handle<Image> = asset_server.load(
		config::assets::CHARACTER,
	);

	Sprite::from_atlas_image(
		texture,
		TextureAtlas {
			layout: texture_atlas_layouts.add(
				internal::sprite_layout(),
			),
			index: internal::indices::DOWN_IDLE.0,
		},
	)
}

pub fn load_character(
	commands: &mut Commands,
	asset_server: &AssetServer,
	texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
	let sprite = load_sprite(asset_server, texture_atlas_layouts);

	commands.spawn(internal::PlayerBundle {
		transform: Transform::from_scale(Vec3::splat(2.0)),
		sprite,
		player: internal::Player,
		facing: internal::PlayerFacing::Down,
		state: internal::PlayerState::Idle,
		animation_indices: internal::indices::DOWN_IDLE,
		animation_timer: internal::timers::idle(),
	});
}
