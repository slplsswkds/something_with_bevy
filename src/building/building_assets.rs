use bevy::asset::AssetPath;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PreviewBuildingHandle(pub Option<Handle<Scene>>);

#[derive(Resource)]
pub struct BuildingAssets {
    pub foundation: BuildingsGroup,
    pub beam: BuildingsGroup,
    pub floor: BuildingsGroup,
    pub wall: BuildingsGroup,
    pub gable: BuildingsGroup,
    pub roof: BuildingsGroup,
}

pub struct BuildingAssetsPack {
    pub name: String,
    pub scene: Handle<Scene>,
    pub _snap_points: Vec<Vec3>,
}

impl BuildingAssetsPack {
    pub fn new(
        bridge: &mut BuildingAssetsInitBridge,
        name: impl Into<String>,
        asset_path: AssetPath,
        snap_points: Vec<Vec3>,
    ) -> Self {
        Self {
            name: name.into(),
            scene: bridge.asset_server.load(asset_path),
            _snap_points: snap_points,
        }
    }
}

pub struct BuildingsGroup(pub Vec<BuildingAssetsPack>);

impl BuildingsGroup {
    fn new(buildings: Vec<BuildingAssetsPack>) -> Self {
        BuildingsGroup(buildings)
    }
    fn empty() -> Self {
        Self::new(Vec::new())
    }
    fn add(mut self, asset: BuildingAssetsPack) -> Self {
        self.0.push(asset);
        self
    }
}

#[derive(SystemParam)]
pub struct BuildingAssetsInitBridge<'w> {
    asset_server: Res<'w, AssetServer>,
}

impl BuildingAssets {
    pub fn load_all(mut bridge: BuildingAssetsInitBridge) -> Self {
        let foundation = load_group_foundation(&mut bridge);
        let gable = load_group_gabble(&mut bridge);
        let floor = load_group_floor(&mut bridge);
        let beam = load_group_beam(&mut bridge);
        let wall = load_group_wall(&mut bridge);
        let roof = load_group_roof(&mut bridge);

        Self {
            foundation,
            beam,
            floor,
            wall,
            gable,
            roof,
        }
    }
}

#[inline]
#[allow(unused_variables, unused_mut)]
fn load_group_foundation(bridge: &mut BuildingAssetsInitBridge) -> BuildingsGroup {
    BuildingsGroup::empty()
}

#[inline]
#[allow(unused_variables, unused_mut)]
fn load_group_beam(mut bridge: &mut BuildingAssetsInitBridge) -> BuildingsGroup {
    BuildingsGroup::empty()
}

#[inline]
fn load_group_floor(mut bridge: &mut BuildingAssetsInitBridge) -> BuildingsGroup {
    BuildingsGroup::empty()
        .add(BuildingAssetsPack::new(
            &mut bridge,
            "Floor 2x2",
            GltfAssetLabel::Scene(0).from_asset("models/floor.gltf"),
            Vec::new(),
        ))
        .add(BuildingAssetsPack::new(
            &mut bridge,
            "Floor 1x1",
            GltfAssetLabel::Scene(1).from_asset("models/floor.gltf"),
            Vec::new(),
        ))
}

#[inline]
fn load_group_wall(mut bridge: &mut BuildingAssetsInitBridge) -> BuildingsGroup {
    BuildingsGroup::empty().add(BuildingAssetsPack::new(
        &mut bridge,
        "Wall 2x2",
        GltfAssetLabel::Scene(0).from_asset("models/wall.gltf"),
        Vec::new(),
    ))
}

#[inline]
#[allow(unused_variables, unused_mut)]
fn load_group_gabble(mut bridge: &mut BuildingAssetsInitBridge) -> BuildingsGroup {
    BuildingsGroup::empty()
}

#[inline]
fn load_group_roof(mut bridge: &mut BuildingAssetsInitBridge) -> BuildingsGroup {
    BuildingsGroup::empty().add(BuildingAssetsPack::new(
        &mut bridge,
        "Roof 2x2 45",
        GltfAssetLabel::Scene(0).from_asset("models/roof.gltf"),
        Vec::new(),
    ))
}
