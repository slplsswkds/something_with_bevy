use crate::building::building_assets::PreviewBuildingHandle;
use super::building_assets::BuildingAssets;
use super::{BuildingMode, ChangeBuildingModeEvent};
use crate::universal_camera_controller::{UniCamChangeStateEvent, UniCamState};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_egui::{egui, EguiContexts};

pub fn enter_building_menu(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut evw_change_universal_cam: EventWriter<UniCamChangeStateEvent>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = true;
    evw_change_universal_cam.send(UniCamChangeStateEvent(UniCamState::Disabled));
}

pub fn building_menu(
    mut contexts: EguiContexts,
    mut building_assets: ResMut<BuildingAssets>,
    mut evw_change_build_mode: EventWriter<ChangeBuildingModeEvent>,
    mut preview_building_handle: ResMut<PreviewBuildingHandle>,
) {
    let mut go_build = || {
        evw_change_build_mode.send(ChangeBuildingModeEvent(BuildingMode::Building));
    };

    let font_data = include_bytes!(r"/WDBlueHDD/fonts/jmh_typewriter/JMH Typewriter.ttf").to_vec();

    egui::Window::new("Building Menu").show(contexts.ctx_mut(), |ui| {
        ui.collapsing("Floor", |ui| {
            ui.button("Floor 2x2").clicked().then(|| {
                preview_building_handle.0 = Some(building_assets.floor.floor_2x2.clone());
                go_build();
            });
            ui.button("Floor 1x1").clicked().then(|| {
                preview_building_handle.0 = Some(building_assets.floor.floor_1x1.clone());
                go_build();
            });
        });
        ui.collapsing("Wall", |ui| {
            ui.button("Wall 2x2").clicked().then(|| {
                preview_building_handle.0 = Some(building_assets.wall.wall_2x2.clone());
                go_build();
            });
        });
        ui.collapsing("Roof", |ui| {
            ui.button("Roof 2x2 45°").clicked().then(|| {
                preview_building_handle.0 = Some(building_assets.roof.roof_2x2_45.clone());
                go_build();
            });
        });
    });
}

// pub fn building_menu(
//     mut contexts: EguiContexts,
//     mut building_assets: ResMut<BuildingAssets>,
//     mut evw_change_build_mode: EventWriter<ChangeBuildingModeEvent>,
// ) {
//     let mut go_build = |asset: &Handle<Scene>| {
//         building_assets.preview_obj = Some(asset.clone());
//         evw_change_build_mode.send(ChangeBuildingModeEvent(BuildingMode::Building));
//     };
//
//     egui::Window::new("Building Menu").show(contexts.ctx_mut(), |ui| {
//         ui.collapsing("Floor", |ui| {
//             ui.button("Floor 2x2").clicked().then(|| go_build(&building_assets.floor.floor_2x2));
//             ui.button("Floor 1x1").clicked().then(|| go_build(&building_assets.floor.floor_1x1));
//         });
//         ui.collapsing("Wall", |ui| {
//             ui.button("Wall 2x2").clicked().then(|| go_build(&building_assets.wall.wall_2x2));
//         });
//         ui.collapsing("Roof", |ui| {
//             ui.button("Roof 2x2 45°").clicked().then(|| go_build(&building_assets.roof.roof_2x2_45));
//         });
//     });
// }

pub fn exit_building_menu(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut evw_change_camera_controller_state: EventWriter<UniCamChangeStateEvent>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    evw_change_camera_controller_state.send(UniCamChangeStateEvent(UniCamState::Enabled));
}
