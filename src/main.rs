use bevy::core_pipeline::bloom::Bloom;
use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;
use bevy::render::{
    settings::{Backends, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                // #[cfg(target_os = "windows")]
                // backends: Some(Backends::DX12),
                // #[cfg(target_os = "linux")]
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        }))
        // .insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 512 })
        // .insert_resource(bevy::pbr::PointLightShadowMap { size: 512 })
        .add_systems(Startup, setup_tmp_world_env)
        .add_systems(Update, rotation_system)
        .run();
}

fn setup_tmp_world_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let material_dir: PathBuf = PathBuf::from("Pond Side Grassy and Muddy Land 2k");
    // let color = material_dir.join("color.png");
    // let normal = material_dir.join("normal_opengl.png");
    // let ao = material_dir.join("ao.png");
    // let metallic_roughness = material_dir.join("metallic_roughness.png");

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

    // for x in -5..5 {
    //     for z in -5..5 {
    //         commands.spawn((
    //             Mesh3d(mesh_handle.clone()),
    //             MeshMaterial3d(material.clone()),
    //             Transform::from_xyz((x * 2) as f32, 0.0, (z * 2) as f32)
    //                 .with_scale(Vec3::splat(2.0)),
    //         ));
    //     }
    // }

    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::splat(0.0)).with_scale(Vec3::splat(20.0)),
    ));

    // Cube
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::default())),
    //     MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
    //     Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

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

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(-7.0, 10.0, -7.0).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom::NATURAL,
        Msaa::default(),
    ));
}

fn rotation_system(time: Res<Time>, mut query: Query<&mut Transform, Without<Camera>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_secs() / 10.0)); // Обертання навколо осі Y
    }
}
