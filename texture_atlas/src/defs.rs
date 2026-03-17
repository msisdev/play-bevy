use bevy::{asset::LoadedFolder, prelude::*};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
pub struct RpgSpriteFolder(pub Handle<LoadedFolder>);
