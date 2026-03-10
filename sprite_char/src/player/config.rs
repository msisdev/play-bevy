pub mod animation_indice {
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

pub mod animation_timer {
	use crate::animate::AnimationTimer;
	use bevy::prelude::*;

	pub fn idle() -> AnimationTimer {
		AnimationTimer(Timer::from_seconds(0.7, TimerMode::Repeating))
	}

	pub fn walk() -> AnimationTimer {
		AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating))
	}
}

pub mod asset {
	use bevy::prelude::*;

	pub const DEFAULT: &str = "sprout_lands/char/Basic Charakter Spritesheet.png";

	pub fn default_layout() -> TextureAtlasLayout {
		TextureAtlasLayout::from_grid(
			UVec2::splat(48),
			4,
			4,
			None,
			None,
		)
	}
}

pub mod dynamic {
	pub const WALK_SPEED: f32 = 120.0;
	pub const SCALE: f32 = 3.0;
}
