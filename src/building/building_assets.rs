use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(Resource)]
pub struct BuildingAssets {
    pub preview_obj: Option<Handle<Scene>>,
    pub foundation: Foundation,
    pub beam: Beam,
    pub floor: Floor,
    pub wall: Wall,
    pub gable: Gable,
    pub roof: Roof,
}

pub struct Foundation {
    pub foundation_2x2: Handle<Scene>,
}

pub struct Beam {
    pub beam_2m: Handle<Scene>,
}

pub struct Floor {
    pub floor_2x2: Handle<Scene>,
    pub floor_1x1: Handle<Scene>,
}

pub struct Wall {
    pub wall_2x2: Handle<Scene>,
}

pub struct Roof {
    pub roof_2x2_45: Handle<Scene>,
}

pub struct Gable {
    pub gable_2x2_45: Handle<Scene>,
}

trait BuildingAsset {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self;
}

impl BuildingAsset for Foundation {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self {
        let foundation_2x2: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("Foundation 2k/foundation.gltf"));
        Self { foundation_2x2 }
    }
}

impl BuildingAsset for Beam {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self {
        let beam_2m: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/beam.gltf"));
        Self { beam_2m }
    }
}

impl BuildingAsset for Floor {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self {
        let floor_2x2: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/floor.gltf"));
        let floor_1x1: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(1).from_asset("models/floor.gltf"));
        Self {
            floor_2x2,
            floor_1x1,
        }
    }
}

impl BuildingAsset for Wall {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self {
        let wall_2x2: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/wall.gltf"));
        Self { wall_2x2 }
    }
}

impl BuildingAsset for Roof {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self {
        let roof_2x2_45: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("Orange Clay Rooftop Tiles 2k/roof.gltf"));
        Self { roof_2x2_45 }
    }
}

impl BuildingAsset for Gable {
    fn load(bridge: &mut BuildingAssetsInitBridge) -> Self {
        let gable_2x2_45: Handle<Scene> = bridge
            .asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("Gable/gable.gltf"));
        Self { gable_2x2_45 }
    }
}

#[derive(SystemParam)]
pub struct BuildingAssetsInitBridge<'w> {
    asset_server: Res<'w, AssetServer>,
    materials: ResMut<'w, Assets<StandardMaterial>>,
}

impl BuildingAssets {
    pub fn load_all(mut bridge: BuildingAssetsInitBridge) -> Self {
        // let cube_mesh = bridge.meshes.add(Cuboid::from_size(Vec3::splat(1.000001)));
        // let preview_material = bridge.materials.add(Color::srgba(0.5, 0.5, 1.0, 0.5));

        Self {
            preview_obj: None,
            foundation: Foundation::load(&mut bridge),
            beam: Beam::load(&mut bridge),
            floor: Floor::load(&mut bridge),
            wall: Wall::load(&mut bridge),
            gable: Gable::load(&mut bridge),
            roof: Roof::load(&mut bridge),
        }
    }
}
