#![feature(result_option_inspect)]

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use bevy_tutorial::{bullet::*, components::*, resources::*, target::*, tower::*};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

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
        .add_plugin(DebugLinesPlugin::default())
        // Inspector requires that components are `reflect` and `register_type`.
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ComponentsPlugin {})
        .add_plugin(BulletPlugin {})
        .add_plugin(TargetPlugin {})
        .add_plugin(TowerPlugin {})
        // Our system.
        .add_startup_systems((load_assets,).in_base_set(StartupSet::PreStartup))
        .add_startup_systems((spawn_camera, spawn_basic_scene, display_axes))
        .add_system(camera_control)
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
    let hitbox = 0.2;
    for location in [
        Transform::from_xyz(-1.0, 0.2, 1.5),
        Transform::from_xyz(-2.0, 0.2, 1.5),
    ] {
        commands
            .spawn(SceneBundle {
                scene: assets.target_scene.clone(),
                transform: location,
                ..default()
            })
            .insert(TargetBundle::new(
                3.0,
                Vec3::X * target_speed_factor,
                hitbox,
            ))
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

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
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
