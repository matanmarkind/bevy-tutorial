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

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Target {
    pub hitbox: f32,
}
