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
    pub fn load_all(bridge: BuildingAssetsInitBridge) -> Self {
        let foundation = BuildingsGroup::empty();

        let beam = BuildingsGroup::empty();

        let floor = BuildingsGroup::empty()
            .add(BuildingAssetsPack {
                name: String::from("Floor 2x2"),
                _snap_points: Vec::new(),
                scene: bridge
                    .asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("models/floor.gltf")),
            })
            .add(BuildingAssetsPack {
                name: String::from("Floor 2x2"),
                _snap_points: Vec::new(),
                scene: bridge
                    .asset_server
                    .load(GltfAssetLabel::Scene(1).from_asset("models/floor.gltf")),
            });

        let wall = BuildingsGroup::empty().add(BuildingAssetsPack {
            name: String::from("Wall 2x2"),
            _snap_points: Vec::new(),
            scene: bridge
                .asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("models/wall.gltf")),
        });

        let gable = BuildingsGroup::empty();

        let roof = BuildingsGroup::empty().add(BuildingAssetsPack {
            name: String::from("Roof 2x2 45"),
            _snap_points: Vec::new(),
            scene: bridge.asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("Orange Clay Rooftop Tiles 2k/roof.gltf"),
            ),
        });

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
