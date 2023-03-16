use crate::components::*;

use bevy::prelude::*;
use bevy::utils::FloatOrd;

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet {
    pub damage: f32,
}

#[derive(Debug, Bundle, Default)]
pub struct BulletBundle {
    pub velocity: Velocity,
    pub bullet: Bullet,
}

impl BulletBundle {
    pub fn new(velocity: Vec3, damage: f32) -> Self {
        Self {
            velocity: Velocity { val: velocity },
            bullet: Bullet { damage },
        }
    }
}

pub fn update_bullets(
    mut commands: Commands,
    mut bullets: Query<(
        Entity,
        &Velocity,
        &mut Transform,
        &mut Lifetime,
        &GlobalTransform,
        &Bullet,
    )>,
    mut targets: Query<(&mut Health, &GlobalTransform, &Target)>,
    time: Res<Time>,
) {
    for (entity, velocity, mut transform, mut lifetime, global_transform, bullet) in &mut bullets {
        lifetime.timer.tick(time.delta());

        // Despawn bullets.
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        transform.translation += velocity.val * time.delta_seconds();

        let closest_target =
            targets
                .iter_mut()
                .min_by_key(|(_health, target_transform, _target)| {
                    FloatOrd(
                        global_transform
                            .translation()
                            .distance(target_transform.translation()),
                    )
                });

        let (mut target_health, target_location, hitbox) = match closest_target {
            None => continue,
            Some((health, transform, target)) => (health, transform.translation(), target.hitbox),
        };
        let distance = global_transform.translation().distance(target_location);
        if distance < hitbox {
            target_health.val -= bullet.damage;
            commands.entity(entity).despawn_recursive();
        }
    }
}
