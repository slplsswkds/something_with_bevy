use super::bridge::ReadyMaterialsInitBridge;
use super::{ReadyMaterialsResource, ReadyMaterialsState};
use bevy::prelude::{AssetServer, Commands, Handle, NextState, Res, ResMut, StandardMaterial};

pub fn init_materials(
    mut commands: Commands,
    mut state: ResMut<NextState<ReadyMaterialsState>>,
    bridge: ReadyMaterialsInitBridge,
) {
    let ready_materials = ReadyMaterialsResource::init(bridge);
    commands.insert_resource(ready_materials);
    state.set(ReadyMaterialsState::Loading)
}

pub fn track_load_progress(
    server: Res<AssetServer>,
    mut state: ResMut<NextState<ReadyMaterialsState>>,
    ready_materials_resource: Res<ReadyMaterialsResource>,
) {
    use bevy::asset::LoadState;

    let mut unready_materials_counter = 0;

    let mut get_load_states = |asset: &Handle<StandardMaterial>| {
        match server.get_load_state(asset.id()) {
            Some(LoadState::Failed(_error)) => {}
            Some(LoadState::Loaded) => {}
            _ => {
                // NotLoaded/Loading: ще не готово
                unready_materials_counter += 1;
            }
        }
    };

    get_load_states(&ready_materials_resource.grassy_land);

    if unready_materials_counter == 0 {
        state.set(ReadyMaterialsState::Ready)
    }
}
