use crate::bullet::*;
use crate::components::*;
use crate::resources::*;

use bevy::prelude::*;
use bevy::utils::FloatOrd;

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_spawn_offset: Vec3,
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

pub fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if !tower.shooting_timer.just_finished() {
            continue;
        }

        let bullet_spawn_loc = transform.translation() + tower.bullet_spawn_offset;
        let towards_enemy = targets
            .iter()
            .min_by_key(|target| FloatOrd(bullet_spawn_loc.distance(target.translation())))
            .map(|closest_target| closest_target.translation() - bullet_spawn_loc);
        let direction = match towards_enemy {
            Some(enemy) => enemy.normalize(),
            None => continue,
        };
        let speed = 5.0;
        let damage = 1.0;
        commands.entity(entity).with_children(|child_builder| {
            child_builder
                .spawn(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    // Since spawning as a child, give the transform relative to the parent.
                    // https://bevy-cheatbook.github.io/features/transforms.html#transform
                    transform: Transform::from_translation(tower.bullet_spawn_offset),
                    ..default()
                })
                .insert(BulletBundle::new(direction * speed, damage))
                .insert(Lifetime {
                    timer: Timer::from_seconds(10.0, TimerMode::Once),
                })
                .insert(Name::new("Bullet"));
        });
    }
}
