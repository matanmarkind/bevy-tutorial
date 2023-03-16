use crate::components::*;

use bevy::prelude::*;

pub fn update_targets(
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
