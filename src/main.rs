mod building;
mod main_menu;
mod ready_materials;
mod settings;
mod universal_camera_controller;

use crate::ready_materials::{ReadyMaterialsPlugin, ReadyMaterialsResource};
use crate::universal_camera_controller::SphericalCamera;
use bevy::core_pipeline::{bloom::Bloom, motion_blur::MotionBlur};
use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use bevy::window::*;
use bevy_egui::EguiPlugin;
use building::BuildingPlugin;
use main_menu::MainMenuPlugin;
use settings::GameSettingsPlugin;
use universal_camera_controller::{UniCamController, UniCamPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        cursor_options: CursorOptions {
                            grab_mode: CursorGrabMode::Locked,
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GameSettingsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(UniCamPlugin)
        .add_plugins(BuildingPlugin)
        .add_plugins(ReadyMaterialsPlugin)
        .add_systems(Startup, setup_tmp_world_env)
        .add_systems(Startup, spawn_wall)
        .run();
}

fn setup_tmp_world_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ready_materials: Res<ReadyMaterialsResource>,
) {
    let mut mesh = Plane3d::default().mesh().size(1.0, 1.0).build();
    mesh.generate_tangents()
        .expect("Failed to generate tangents");

    let mesh_handle = meshes.add(mesh);
    let material = ready_materials.grassy_land.clone();

    // Ground
    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::splat(0.0)).with_scale(Vec3::splat(2.0)),
    ));

    let map_size = 50 / 2; // 50 m. 1px = 1m.
    for x in -map_size..map_size {
        for z in -map_size..map_size {
            // Ground
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::new(x as f32 * 2.0, 0.0, z as f32 * 2.0))
                    .with_scale(Vec3::splat(2.0)),
            ));
        }
    }

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 5_000_000.0,
            color: Color::srgb(1.0, 0.95, 0.92),
            ..default()
        },
        Transform::from_xyz(3.0, 6.0, 1.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        PerspectiveProjection {
            fov: 120.0_f32.to_radians(),
            ..default()
        },
        Bloom::NATURAL,
        Msaa::default(),
        UniCamController::from(SphericalCamera::default()),
        MotionBlur {
            shutter_angle: 0.5,
            samples: 1,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0).look_at(Vec3::new(-1.0, 1.0, 0.0), Vec3::Y),
    ));
}

fn spawn_wall(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wall_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wall.gltf"));
    commands.spawn((
        SceneRoot(wall_scene.clone()),
        Transform::from_translation(Vec3::new(-1.0, 1.0, 0.0)),
    ));
}
