use bevy::prelude::*;

// Resources.
#[derive(Resource, Debug)]
pub struct GameAssets {
    pub bullet_scene: Handle<Scene>,
}
