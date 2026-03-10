use bevy::prelude::*;

use crate::animate;

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
		transform: Transform::from_scale(Vec3::splat(config::dynamic::SCALE)),
		sprite,
		animation_indices: config::animation_indice::DOWN_IDLE,
		animation_timer: config::animation_timer::idle(),
		action: component::PlayerAction::Idle,
		facing: component::PlayerFacing::Down,
	});
}

pub fn move_player(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	player_bundle: Single<(
		&mut Transform,
		&mut component::PlayerAction,
		&mut component::PlayerFacing,
		&mut animate::AnimationIndices,
		&mut animate::AnimationTimer,
	), With<component::Player>>,
	time: Res<Time>,
) {
	let (
		mut player_transform,
		mut player_action,
		mut player_facing,
		mut player_animation_indices,
		mut player_animation_timer,
	) = player_bundle.into_inner();
	let mut direction = Vec2::ZERO;
	let mut new_player_action = player_action.clone();
	let mut new_player_facing = *player_facing;
	{
		if keyboard_input.pressed(KeyCode::ArrowUp) {
			direction += Vec2::Y;
			new_player_action = component::PlayerAction::Walk;
			new_player_facing = component::PlayerFacing::Up;
		}
		if keyboard_input.pressed(KeyCode::ArrowDown) {
			direction += Vec2::NEG_Y;
			new_player_action = component::PlayerAction::Walk;
			new_player_facing = component::PlayerFacing::Down;
		}
		if keyboard_input.pressed(KeyCode::ArrowLeft) {
			direction += Vec2::NEG_X;
			new_player_action = component::PlayerAction::Walk;
			new_player_facing = component::PlayerFacing::Left;
		}
		if keyboard_input.pressed(KeyCode::ArrowRight) {
			direction += Vec2::X;
			new_player_action = component::PlayerAction::Walk;
			new_player_facing = component::PlayerFacing::Right;
		}

		// Only go idle when no movement keys remain held
		if direction == Vec2::ZERO {
			new_player_action = component::PlayerAction::Idle;
		}

		// Update animation whenever the action or facing direction changes
		if *player_action.as_ref() != new_player_action || *player_facing != new_player_facing {
			(*player_animation_indices, *player_animation_timer) = match (new_player_action, new_player_facing) {
				(PlayerAction::Idle, PlayerFacing::Down) => (config::animation_indice::DOWN_IDLE, config::animation_timer::idle()),
				(PlayerAction::Idle, PlayerFacing::Up) => (config::animation_indice::UP_IDLE, config::animation_timer::idle()),
				(PlayerAction::Idle, PlayerFacing::Left) => (config::animation_indice::LEFT_IDLE, config::animation_timer::idle()),
				(PlayerAction::Idle, PlayerFacing::Right) => (config::animation_indice::RIGHT_IDLE, config::animation_timer::idle()),
				(PlayerAction::Walk, PlayerFacing::Down) => (config::animation_indice::DOWN_WALK, config::animation_timer::walk()),
				(PlayerAction::Walk, PlayerFacing::Up) => (config::animation_indice::UP_WALK, config::animation_timer::walk()),
				(PlayerAction::Walk, PlayerFacing::Left) => (config::animation_indice::LEFT_WALK, config::animation_timer::walk()),
				(PlayerAction::Walk, PlayerFacing::Right) => (config::animation_indice::RIGHT_WALK, config::animation_timer::walk()),
			}
		}
		*player_facing = new_player_facing;
	}

	if direction.length() > 0.0 {
		direction = direction.normalize();
	}

	let new_player_position = player_transform.translation.xy()
		+ direction * config::dynamic::WALK_SPEED * time.delta_secs();

	// Update
	player_transform.translation = new_player_position.extend(player_transform.translation.z);
	*player_action = new_player_action;
}
