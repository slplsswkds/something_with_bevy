use crate::ready_materials::bridge::ReadyMaterialsInitBridge;
use bevy::image::ImageLoaderSettings;
use bevy::prelude::{default, Handle, Resource, StandardMaterial};
use std::path::PathBuf;

#[derive(Resource)]
pub struct ReadyMaterialsResource {
    pub grassy_land: Handle<StandardMaterial>,
}

impl ReadyMaterialsResource {
    pub fn init(bridge: ReadyMaterialsInitBridge) -> Self {
        Self {
            grassy_land: load_material(bridge, "Pond Side Grassy and Muddy Land 2k"),
        }
    }
}

fn load_material(
    bridge: ReadyMaterialsInitBridge,
    material_dir_name: impl Into<PathBuf> + AsRef<std::path::Path>,
) -> Handle<StandardMaterial> {
    let material_path = PathBuf::from("materials").join(material_dir_name);

    let color = material_path.join("color.ktx2");
    let normal = material_path.join("normal_opengl.ktx2");
    let ao = material_path.join("ao.ktx2");
    let metallic_roughness = material_path.join("metallic_roughness.ktx2");

    let mut materials = bridge.materials;
    let asset_server = bridge.asset_server;

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

    material
}
