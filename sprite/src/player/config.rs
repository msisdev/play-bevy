pub mod animation_indice {
	use crate::animate::AnimationIndices;

	pub const DOWN_IDLE: AnimationIndices = AnimationIndices(0, 1);
}

pub mod animation_timer {
	use crate::animate::AnimationTimer;
	use bevy::prelude::*;

	pub fn idle() -> AnimationTimer {
		AnimationTimer(Timer::from_seconds(0.7, TimerMode::Repeating))
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
