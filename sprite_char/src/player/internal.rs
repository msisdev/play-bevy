use bevy::prelude::*;

use crate::animate::{AnimationIndices, AnimationTimer};

pub fn sprite_layout() -> TextureAtlasLayout {
	TextureAtlasLayout::from_grid(
		UVec2::splat(48),
		4,
		4,
		None,
		None,
	)
}

pub mod indices {
	use crate::animate::AnimationIndices;

	pub const DOWN_IDLE: AnimationIndices = AnimationIndices(0, 1);
	pub const DOWN_WALK: AnimationIndices = AnimationIndices(2, 3);
	pub const UP_IDLE: AnimationIndices = AnimationIndices(4, 5);
	pub const UP_WALK: AnimationIndices = AnimationIndices(6, 7);
	pub const LEFT_IDLE: AnimationIndices = AnimationIndices(8, 9);
	pub const LEFT_WALK: AnimationIndices = AnimationIndices(10, 11);
	pub const RIGHT_IDLE: AnimationIndices = AnimationIndices(12, 13);
	pub const RIGHT_WALK: AnimationIndices = AnimationIndices(14, 15);
}

pub mod timers {
	use crate::animate::AnimationTimer;
	use bevy::prelude::*;

	pub fn idle() -> AnimationTimer {
		AnimationTimer(Timer::from_seconds(0.7, TimerMode::Repeating))
	}
	pub fn walk() -> AnimationTimer {
		AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
	}
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
	/* world */
	pub sprite: Sprite,
	pub transform: Transform,

	/* label */
	pub player: Player,

	/* states */
	pub facing: PlayerFacing,
	pub state: PlayerState,

	/* sprite */
	pub animation_indices: AnimationIndices,
	pub animation_timer: AnimationTimer,
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
