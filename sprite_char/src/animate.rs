use bevy::prelude::*;

/// First and last frame indices for a sprite animation.
#[derive(Component)]
pub struct AnimationIndices(pub usize, pub usize);

/// Timer for sprite animation frame changes.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
		&AnimationIndices,
		&mut AnimationTimer,
		&mut Sprite
	)>,
) {
    for (
        indices,
        mut timer,
        mut sprite
    ) in query.iter_mut() {
		// Progress the timer
        timer.tick(time.delta());

		// 
        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.1 {
                indices.0
            } else {
                atlas.index + 1
            };
        }
    }
}
