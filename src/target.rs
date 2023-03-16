use crate::components::*;

use bevy::prelude::*;

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Target {
    pub hitbox: f32,
}

#[derive(Debug, Bundle, Default)]
pub struct TargetBundle {
    pub velocity: Velocity,
    pub health: Health,
    pub target: Target,
}

impl TargetBundle {
    pub fn new(health: f32, velocity: Vec3, hitbox: f32) -> Self {
        Self {
            velocity: Velocity { val: velocity },
            health: Health { val: health },
            target: Target { hitbox },
        }
    }
}

pub struct TargetPlugin {}

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>().add_system(update_targets);
    }
}

fn update_targets(
    mut commands: Commands,
    mut targets: Query<(Entity, &mut Transform, &Health, &Velocity), With<Target>>,
    time: Res<Time>,
) {
    // Move live targets.
    for (entity, mut transform, health, velocity) in targets.iter_mut() {
        if health.val <= 0.0 {
            commands.entity(entity).despawn_recursive();
            return; // Exits this scope, not the entire iteration.
        }

        transform.translation += velocity.val * time.delta_seconds();
    }
}
