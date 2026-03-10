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

	/* state */
	pub action: PlayerAction,
	pub facing: PlayerFacing,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PlayerAction {
	Idle,
	Walk,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PlayerFacing {
	Down,
	Up,
	Left,
	Right,
}
