use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{AssetServer, Res, ResMut};

#[derive(SystemParam)]
pub struct ReadyMaterialsInitBridge<'w> {
    pub asset_server: Res<'w, AssetServer>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}
