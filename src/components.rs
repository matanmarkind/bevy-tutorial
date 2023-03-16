use bevy::prelude::*;

// Shared components.
#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Health {
    pub val: f32,
}

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub val: Vec3,
}

pub struct ComponentsPlugin {}

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lifetime>()
            .register_type::<Health>()
            .register_type::<Velocity>();
    }
}
