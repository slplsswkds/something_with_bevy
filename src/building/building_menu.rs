use super::building_assets::BuildingAssets;
use super::{BuildingMode, ChangeBuildingModeEvent};
use crate::universal_camera_controller::{UniCamState, UniCamChangeStateEvent};
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
) {
    let mut go_build = || {
        evw_change_build_mode.send(ChangeBuildingModeEvent(BuildingMode::Building));
    };

    egui::Window::new("Building Menu").show(contexts.ctx_mut(), |ui| {
        ui.collapsing("Roof", |ui| {
            ui.button("Roof 2x2 45°").clicked().then(|| {
                building_assets.preview_obj = Some(building_assets.roof.roof_2x2_45.clone());
                go_build();
            });
        });
        ui.collapsing("Wall", |ui| {
            ui.button("Wall 2x2").clicked().then(|| {
                building_assets.preview_obj = Some(building_assets.wall.wall_2x2.clone());
                go_build();
            });
        });
    });
}

pub fn exit_building_menu(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut camera_controller_state: ResMut<NextState<UniCamState>>,
    mut evw_change_camera_controller_state: EventWriter<UniCamChangeStateEvent>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    evw_change_camera_controller_state.send(UniCamChangeStateEvent(UniCamState::Enabled));
}
