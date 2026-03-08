use bevy::prelude::*;
use crate::{animate::AnimationIndices};
use super::internal::*;

use super::internal::Player;

pub fn move_player(
	keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Single<
		(&mut Transform, &mut PlayerFacing, &mut PlayerState),
		With<Player>
	>,
    time: Res<Time>,
) {
	let (
		mut transform,
		mut player_facing,
		mut player_state,
	) = player_query.into_inner();

	let mut direction = Vec2::ZERO;
	if keyboard_input.pressed(KeyCode::ArrowUp) {
		direction.y += 1.;
	}
	if keyboard_input.pressed(KeyCode::ArrowDown) {
		direction.y -= 1.;
	}
	if keyboard_input.pressed(KeyCode::ArrowLeft) {
		direction.x -= 1.;
	}
	if keyboard_input.pressed(KeyCode::ArrowRight) {
		direction.x += 1.;
	}

	if direction == Vec2::ZERO {
		player_state = PlayerState::Idle;
	} else {
		transform.translation += (direction * time.delta_secs() * 100.).extend(0.);
	}
}
