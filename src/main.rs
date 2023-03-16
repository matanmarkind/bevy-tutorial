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
        // Inspector setup.
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .register_type::<Lifetime>()
        .register_type::<Target>()
        .register_type::<Velocity>()
        .register_type::<Bullet>()
        // Our system.
        .add_startup_systems((spawn_camera, spawn_basic_scene, load_assets, display_axes))
        .add_systems((tower_shooting, update_bullets, update_targets))
        .run()
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            bullet_spawn_offset: Vec3::new(0.0, 0.2, 0.5),
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
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
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

fn load_assets(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
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
