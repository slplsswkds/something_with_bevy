use super::building_assets::{BuildingAssets, BuildingsGroup};
use super::{BuildingMode, ChangeBuildingModeEvent};
use crate::building::building_assets::PreviewBuildingHandle;
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
    building_assets: Res<BuildingAssets>,
    mut evw_change_build_mode: EventWriter<ChangeBuildingModeEvent>,
    mut preview_building_handle: ResMut<PreviewBuildingHandle>,
) {
    let mut show_building_category =
        |ui: &mut egui::Ui, category_name: &str, buildings: &BuildingsGroup| {
            ui.collapsing(category_name, |ui| {
                for building in &buildings.0 {
                    ui.button(building.name.clone()).clicked().then(|| {
                        preview_building_handle.0 = Some(building.scene.clone());
                        evw_change_build_mode.send(ChangeBuildingModeEvent(BuildingMode::Building));
                    });
                }
            });
        };

    egui::Window::new("Building Menu").show(contexts.ctx_mut(), |ui| {
        show_building_category(ui, "Foundation", &building_assets.foundation);
        show_building_category(ui, "Beam", &building_assets.beam);
        show_building_category(ui, "Floor", &building_assets.floor);
        show_building_category(ui, "Wall", &building_assets.wall);
        show_building_category(ui, "Gable", &building_assets.gable);
        show_building_category(ui, "Roof", &building_assets.roof);
    });
}

pub fn exit_building_menu(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut evw_change_camera_controller_state: EventWriter<UniCamChangeStateEvent>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    evw_change_camera_controller_state.send(UniCamChangeStateEvent(UniCamState::Enabled));
}
