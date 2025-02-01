mod building;
mod universal_camera_controller;

use crate::building::prelude::*;
use bevy::core_pipeline::bloom::Bloom;
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use bevy::window::CursorGrabMode::Locked;
use bevy::window::*;
use std::path::PathBuf;
use universal_camera_controller::prelude::*;

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
                            grab_mode: Locked,
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(UniversalCameraControllerPlugin)
        .add_plugins(BuildingPlugin)
        .add_systems(Startup, setup_tmp_world_env)
        .add_systems(Startup, spawn_wall)
        .add_systems(Update, cast_ray_from_camera)
        .run();
}

fn setup_tmp_world_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let material_dir: PathBuf = PathBuf::from("Pond Side Grassy and Muddy Land 2k");

    let color = material_dir.join("color.ktx2"); // toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf srgb --assign_oetf srgb --zcmp 20 color.ktx2 color.png
    let normal = material_dir.join("normal_opengl.ktx2"); // toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf srgb --assign_oetf linear --zcmp 20 normal_opengl.ktx2 normal_opengl.png
    let ao = material_dir.join("ao.ktx2"); // toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf linear --assign_oetf linear --zcmp 20 ao.ktx2 ao.png
    let metallic_roughness = material_dir.join("metallic_roughness.ktx2"); // toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf linear --assign_oetf linear --zcmp 20 metallic_roughness.ktx2 metallic_roughness.png

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(
            asset_server.load_with_settings(color, |settings: &mut ImageLoaderSettings| {
                settings.is_srgb = true
            }),
        ),
        occlusion_texture: Some(
            asset_server.load_with_settings(ao, |settings: &mut ImageLoaderSettings| {
                settings.is_srgb = false
            }),
        ),
        normal_map_texture: Some(
            asset_server.load_with_settings(normal, |settings: &mut ImageLoaderSettings| {
                settings.is_srgb = true
            }),
        ),
        metallic_roughness_texture: Some(
            asset_server
                .load_with_settings(metallic_roughness, |settings: &mut ImageLoaderSettings| {
                    settings.is_srgb = false
                }),
        ),
        metallic: 1.0,
        perceptual_roughness: 1.0,
        ..default()
    });

    let mut mesh = Plane3d::default().mesh().size(1.0, 1.0).build();
    mesh.generate_tangents()
        .expect("Failed to generate tangents");

    let mesh_handle = meshes.add(mesh);

    // // Cube
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::default())),
    //     MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

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
        Bloom::NATURAL,
        Msaa::default(),
        UniversalCameraController::spherical_camera(),
    ));
}

fn spawn_wall(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("Medieval Timbered Wall 2k/wall.gltf")),
        ),
        Transform::from_translation(Vec3::new(-1.0, 1.0, 0.0)),
    ));
}

fn cast_ray_from_camera(
    camera_query: Query<&GlobalTransform, With<UniversalCameraController>>,
    mut ray_cast: MeshRayCast,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let origin = camera_transform.translation();
        let direction = camera_transform.forward();

        let ray = Ray3d::new(origin, direction);

        let settings = RayCastSettings::default();

        let hits = ray_cast.cast_ray(ray, &settings);

        for (entity, hit) in hits {
            println!(
                "The ray hit the object {:?} at point {:?}",
                entity, hit.point
            );
        }
    }
}
