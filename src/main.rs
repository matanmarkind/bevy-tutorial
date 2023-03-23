#![feature(result_option_inspect)]
#![feature(option_result_contains)]

use std::time::Duration;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::WindowResolution;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_rapier3d::prelude::*;

use bevy_tutorial::{bullet::*, components::*, resources::*, target::*, tower::*};
use derivative::Derivative;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const DEFAULT_DEBOUNCE: GapTimer = GapTimer::new(Duration::from_millis(200));

// Timer used to check for a specific gap in time.
// Should only be ticked by a single system.
#[derive(Debug, Default, Reflect)]
pub struct GapTimer {
    pub last: Duration,
    pub gap: Duration,
}

impl GapTimer {
    pub const fn new(gap: Duration) -> GapTimer {
        GapTimer {
            last: Duration::ZERO,
            gap,
        }
    }

    // Returns the number of ticks which have passed since the last full tick. If at least 1 tick
    // has passed, resets the timer.
    pub fn tick(&mut self, time: Duration) -> u32 {
        let diff = time - self.last;
        if diff < self.gap {
            return 0;
        }
        self.last = time;
        (diff.as_nanos() / self.gap.as_nanos()) as u32
    }
}

#[derive(Debug, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct MousedOverBlinker {
    pub entity: Option<Entity>,
    pub original_visibility: Visibility,
    pub timer: GapTimer,
}

#[derive(Debug, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct MousedOverEntity {
    pub entity: Option<Entity>,
}

#[derive(Derivative, Resource, Reflect)]
#[derivative(Debug, Default)]
#[reflect(Resource)]
pub struct SelectedEntity {
    pub entity: Option<Entity>,
    #[derivative(Default(value = "DEFAULT_DEBOUNCE"))]
    pub debounce: GapTimer,
    pub blinker: GapTimer,
    pub original_visibility: Visibility,
}

#[derive(Derivative, Resource, Reflect)]
#[derivative(Debug, Default)]
#[reflect(Resource)]
pub struct SpacebarTimer {
    #[derivative(Default(value = "DEFAULT_DEBOUNCE"))]
    pub debounce: GapTimer,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "Bevy Tower Defense".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(DebugLinesPlugin::default())
        // Rapier
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .register_type::<RigidBody>()
        // Inspector requires that components are `reflect` and `register_type`.
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ComponentsPlugin {})
        .add_plugin(BulletPlugin {})
        .add_plugin(TargetPlugin {})
        .add_plugin(TowerPlugin {})
        .register_type::<MousedOverBlinker>()
        .register_type::<MousedOverEntity>()
        .register_type::<SelectedEntity>()
        .register_type::<SpacebarTimer>()
        // Our system.
        .insert_resource(MousedOverBlinker {
            timer: GapTimer::new(Duration::from_millis(300)),
            ..default()
        })
        .insert_resource(MousedOverEntity::default())
        .insert_resource(SelectedEntity {
            blinker: GapTimer::new(Duration::from_millis(100)),
            ..default()
        })
        .insert_resource(SpacebarTimer::default())
        .add_startup_systems((load_assets,).in_base_set(StartupSet::PreStartup))
        .add_startup_systems((spawn_camera, spawn_basic_scene, display_axes))
        .add_systems((
            camera_control,
            moused_over_entity,
            pause,
            select_moused_over,
            // Consider only having 1 of these at a time.
            // blink_moused_over,
            update_selected_entity,
        ))
        .run()
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                subdivisions: 0,
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(SceneBundle {
            scene: assets.tower_scene.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            bullet_spawn_offset: Vec3::new(0.0, 1.4, 0.0),
        })
        .insert(RigidBody::Fixed) // Seems needed for the Collider transform.
        .with_children(|child_cmd| {
            child_cmd
                .spawn(Collider::cylinder(0.7, 0.6))
                .insert(Transform::from_xyz(0.0, 0.7, 0.0))
                .insert(Name::new("Hitbox"));
        })
        .insert(Name::new("Tower"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));

    let target_speed_factor = 0.5;
    let hitbox = 0.2; // DO NOT SUBMIT - remove for rapier
    for location in [
        Transform::from_xyz(-1.0, 0.45, 1.5),
        Transform::from_xyz(-2.0, 0.45, 1.5),
    ] {
        commands
            .spawn(SceneBundle {
                scene: assets.target_scene.clone(),
                transform: location,
                ..default()
            })
            .insert(TargetBundle::new(
                10.0,
                Vec3::X * target_speed_factor,
                hitbox,
            ))
            .insert(RigidBody::Fixed) // Seems needed for the Collider transform.
            .with_children(|child_cmd| {
                child_cmd
                    .spawn(Collider::ball(0.4))
                    .insert(Transform::from_xyz(0.0, 0.0, 0.0))
                    .insert(Name::new("Hitbox"));
            })
            .insert(Name::new("Target"));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_control(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0; // Remove tilt so we will move parallel to the ground.
    forward = forward.normalize();
    let mut left = camera.left();
    left.y = 0.0; // Remove tilt so we will move parallel to the ground.
    left = left.normalize();

    let speed = 4.0;
    let rotate_speed = 0.5;

    // Use raw time so that we can pause time and still move the camera.
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.raw_delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.raw_delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.raw_delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.raw_delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.raw_delta_seconds());
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.raw_delta_seconds());
    }
}

fn pause(keyboard: Res<Input<KeyCode>>, mut time: ResMut<Time>, mut timer: ResMut<SpacebarTimer>) {
    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    // Use raw_delta so that we can unpause time.
    if timer.debounce.tick(time.raw_elapsed()) == 0 {
        return;
    }

    dbg!("pause -- change state");
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

fn select_moused_over(
    buttons: Res<Input<MouseButton>>,
    moused_over_entity: ResMut<MousedOverEntity>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    // Use raw_delta so that we can select entities when time is paused.
    if selected_entity.debounce.tick(time.raw_elapsed()) == 0 {
        return;
    }

    if selected_entity.entity == moused_over_entity.entity {
        return;
    }

    if let Some(entity) = selected_entity.entity {
        // Restore entity before override.
        if let Ok(mut visibility) = visibility_query.get_mut(entity) {
            *visibility = selected_entity.original_visibility;
        } else {
            // This happens if the target is despawned.
        }
    }

    selected_entity.entity = moused_over_entity.entity;
    dbg!(&selected_entity.entity);
}

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

fn display_axes(mut lines: ResMut<DebugLines>) {
    lines.line_colored(
        Vec3::ZERO,
        Vec3 {
            x: 1e6,
            y: 0.0,
            z: 0.0,
        },
        1e6,
        Color::RED,
    );
    lines.line_colored(
        Vec3::ZERO,
        Vec3 {
            x: 0.0,
            y: 1e6,
            z: 0.0,
        },
        1e6,
        Color::GREEN,
    );
    lines.line_colored(
        Vec3::ZERO,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1e6,
        },
        1e6,
        Color::BLUE,
    );
}

fn moused_over_entity(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    rapier_context: Res<RapierContext>,
    mut moused_over_entity: ResMut<MousedOverEntity>,
    parent_query: Query<&Parent>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let Ok(window) = primary_query.get_single() else {
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Assume only 1 camera.
    let (camera_transform, camera) = camera_query.single();
    let Ray { origin, direction } = camera
        .viewport_to_world(camera_transform, cursor_position)
        .unwrap();

    let max_toi = 400.0;
    let solid = true;

    if let Some((entity, toi)) =
        rapier_context.cast_ray(origin, direction, max_toi, solid, QueryFilter::new())
    {
        // The first collider hit has the entity `entity` and it hit after
        // the ray travelled a distance equal to `ray_dir * toi`.
        let hit_point = origin + direction * toi;
        println!("Entity {:?} hit at point {}", entity, hit_point);

        // Colliders, by my convention, are always the children of the actual entity of interest.
        let Ok(parent_entity) = parent_query.get(entity) else {
            return;
        };
        moused_over_entity.entity = Some(parent_entity.get());
    } else {
        moused_over_entity.entity = None;
    }
}

// Assumes that mouse entity selection is done via rapier colliders, which are the child of the
// actual entity we want to blink.
fn blink_moused_over(
    moused_over_entity: Res<MousedOverEntity>,
    mut moused_over_blinker: ResMut<MousedOverBlinker>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
) {
    let Some(entity) = moused_over_blinker.entity else {
        let Some(new_entity) = moused_over_entity.entity else {
            return;
        };

        // Store the new blinking target.
        let Ok(visibility) = visibility_query.get(new_entity) else {return;};

        moused_over_blinker.entity = Some(new_entity);
        moused_over_blinker.original_visibility = visibility.clone();
        return;
    };

    // Intentionally put this timer after the above flow.
    // Use raw time so that we check selections when time is paused.
    if moused_over_blinker.timer.tick(time.raw_elapsed()) == 0 {
        return;
    }

    // Blink the entity.
    let Ok(mut visibility) = visibility_query.get_mut(entity) else {
        // This happens if the target is despawned.
        moused_over_blinker.entity = None;
        return;
    };

    if *visibility == Visibility::Hidden {
        *visibility = moused_over_blinker.original_visibility;
        // If the entity was hidden, it could not have been found via casting.
        return;
    }

    // Determine whether to remove the target or not.
    if Some(entity) == moused_over_entity.entity {
        *visibility = Visibility::Hidden;
        return;
    }

    // The current entity is visible, but we are not moused over it. Reset.
    *visibility = moused_over_blinker.original_visibility;
    moused_over_blinker.entity = None;
}

fn update_selected_entity(
    mut selected_entity: ResMut<SelectedEntity>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
) {
    // use raw time since we want to allow entity selection when paused.
    if selected_entity.blinker.tick(time.raw_elapsed()) == 0 {
        return;
    }
    let Some(entity) = selected_entity.entity else {
        return;
    };

    // Blink the entity.
    let Ok(mut visibility) = visibility_query.get_mut(entity) else {
        return;
    };

    if *visibility == Visibility::Hidden {
        *visibility = selected_entity.original_visibility;
    } else {
        *visibility = Visibility::Hidden;
    }
}
