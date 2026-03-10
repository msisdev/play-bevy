use bevy::prelude::*;

use super::*;

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

		// Update sprite
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
