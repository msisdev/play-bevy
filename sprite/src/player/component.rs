use bevy::prelude::*;
use crate::animate::component::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
	/* label */
	pub player: Player,

	/* world */
	pub transform: Transform,

	/* sprite */
	pub sprite: Sprite,
	pub animation_indices: AnimationIndices,
	pub animation_timer: AnimationTimer,

	/* states */
	pub facing: PlayerFacing,
	pub state: PlayerState,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerFacing {
	#[default]
	Up,
	Down,
	Left,
	Right,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
	#[default]
	Idle,
	Walking,
}
