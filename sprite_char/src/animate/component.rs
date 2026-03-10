use bevy::prelude::*;

/// First and last frame indices for a sprite animation.
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub struct AnimationIndices(pub usize, pub usize);

/// Timer for sprite animation frame changes.
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
