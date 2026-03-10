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
        if let Some(atlas) = &mut sprite.texture_atlas {
			// If the index is out of the current animation range (e.g. after
			// switching direction), snap back to the first frame immediately.
			if atlas.index < indices.0 || atlas.index > indices.1 {
				atlas.index = indices.0;
			}

			if timer.just_finished() {
				atlas.index = if atlas.index == indices.1 {
					indices.0
				} else {
					atlas.index + 1
				};
			}
        }
    }
}
