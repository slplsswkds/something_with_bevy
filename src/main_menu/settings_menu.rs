use crate::main_menu::ShowSettingsUiState;
use crate::settings::{GameSettings, GameSettingsBridge};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::*;
use egui::Slider;

#[derive(Resource, Default)]
pub struct TmpGameSettings(GameSettings);

pub fn enter_settings_ui(mut commands: Commands, settings: Res<GameSettings>) {
    commands.insert_resource(TmpGameSettings(settings.clone()));
}

pub fn exit_settings_ui(mut commands: Commands) {
    commands.remove_resource::<TmpGameSettings>();
}

#[derive(SystemParam)]
pub struct SettingsUiBridge<'w> {
    tmp_settings: ResMut<'w, TmpGameSettings>,
    settings: ResMut<'w, GameSettings>,
    settings_ui_state: ResMut<'w, NextState<ShowSettingsUiState>>,
    game_settings_bridge: GameSettingsBridge<'w>,
}

pub fn settings_ui(mut contexts: EguiContexts, mut bridge: SettingsUiBridge) {
    egui::Window::new("Game settings").show(contexts.ctx_mut(), |ui| {
        submenu_keyboard(ui, &mut bridge);
        submenu_mouse(ui, &mut bridge);
        submenu_video(ui, &mut bridge);
        ui.separator();
        form_save_or_cancel_or_defaults(ui, &mut bridge);
    });
}

fn submenu_keyboard(ui: &mut Ui, _bridge: &mut SettingsUiBridge) {
    ui.collapsing("Keyboard", |_| {});
}

fn submenu_mouse(ui: &mut Ui, bridge: &mut SettingsUiBridge) {
    let mouse = &mut bridge.tmp_settings.0.mouse;
    ui.collapsing("Mouse", |ui| {
        add_slider(
            ui,
            "Vertical sensitivity",
            &mut mouse.sensitivity_vertical,
            0.0005..=0.005,
        );
        add_slider(
            ui,
            "Horizontal sensitivity",
            &mut mouse.sensitivity_horizontal,
            0.00075..=0.005,
        );
    });
}

fn submenu_video(ui: &mut Ui, _bridge: &mut SettingsUiBridge) {
    ui.collapsing("Video", |_| {});
}

fn form_save_or_cancel_or_defaults(ui: &mut Ui, bridge: &mut SettingsUiBridge) {
    ui.horizontal(|ui| {
        if ui.button("Reset default").clicked() {
            bridge.tmp_settings.0 = GameSettings::default();
        }
        if ui.button("Apply").clicked() {
            apply_settings(bridge);
        }
        if ui.button("Cancel").clicked() {
            bridge.settings_ui_state.set(ShowSettingsUiState::Inactive);
        }
    });
}

fn add_slider(ui: &mut Ui, text: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>) {
    ui.add(Slider::new(value, range).text(text));
}

fn apply_settings(bridge: &mut SettingsUiBridge) {
    *bridge.settings = bridge.tmp_settings.0.clone();
    bridge
        .settings
        .apply_settings(&mut bridge.game_settings_bridge);
    bridge.settings_ui_state.set(ShowSettingsUiState::Inactive);
}
