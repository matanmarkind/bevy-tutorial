use bevy::prelude::*;

// Resources.
#[derive(Resource, Debug)]
pub struct GameAssets {
    pub tower_base_scene: Handle<Scene>,
    pub tower_scene: Handle<Scene>,
    pub tomato_scene: Handle<Scene>,
    pub target_scene: Handle<Scene>,
}
