use crate::main_menu::ShowSettingsUiState;
use crate::universal_camera_controller::{UniCamChangeStateEvent, UniCamState};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_egui::{egui, EguiContexts};

pub fn enter_main_menu(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut evw_change_universal_cam: EventWriter<UniCamChangeStateEvent>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = true;
    evw_change_universal_cam.send(UniCamChangeStateEvent(UniCamState::Disabled));
}

pub fn exit_main_menu() {}

pub fn main_menu(
    mut contexts: EguiContexts,
    mut exit_events: EventWriter<AppExit>,
    current_settings_state: Res<State<ShowSettingsUiState>>,
    mut settings_state: ResMut<NextState<ShowSettingsUiState>>,
) {
    let settings_btn_state = match current_settings_state.get() {
        ShowSettingsUiState::Inactive => true,
        ShowSettingsUiState::Active => false,
    };

    egui::Window::new("Main menu").show(contexts.ctx_mut(), |ui| {
        ui.button("Start").clicked().then(|| {
            // later...
        });
        ui.add_enabled_ui(settings_btn_state, |ui| {
            if ui.button("Settings").clicked() {
                settings_state.set(ShowSettingsUiState::Active)
            }
        });
        ui.separator();
        ui.button("Quit").clicked().then(|| {
            exit_events.send(AppExit::Success);
        });
    });
}
