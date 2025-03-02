use crate::main_menu::ShowSettingsUiState;
use crate::settings::{GameSettings, GameSettingsBridge};
use bevy::prelude::*;
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

pub fn settings_ui(
    mut contexts: EguiContexts,
    mut tmp_settings: ResMut<TmpGameSettings>,
    mut settings: ResMut<GameSettings>,
    mut settings_ui_state: ResMut<NextState<ShowSettingsUiState>>,
    // mut settings: ResMut<GameSettings>,
    mut game_settings_bridge: GameSettingsBridge,
) {
    egui::Window::new("Game settings").show(contexts.ctx_mut(), |ui| {
        ui.collapsing("Keyboard", |ui| {});

        ui.collapsing("Mouse", |ui| {
            ui.add(
                Slider::new(
                    &mut tmp_settings.0.mouse.sensitivity_vertical,
                    0.0005..=0.005,
                )
                .text("Vertical sensitivity"),
            );
            ui.add(
                Slider::new(
                    &mut tmp_settings.0.mouse.sensitivity_horizontal,
                    0.00075..=0.005,
                )
                .text("Horizontal sensitivity"),
            );
        });

        ui.collapsing("Video", |ui| {});

        ui.separator();
        ui.horizontal(|ui| {
            ui.button("Reset default").clicked().then(|| {
                tmp_settings.0 = GameSettings::default();
            });
            ui.button("Apply").clicked().then(|| {
                *settings = tmp_settings.0.clone();
                settings.apply_settings(&mut game_settings_bridge);
                settings_ui_state.set(ShowSettingsUiState::Inactive);
            });
            ui.button("Cancel")
                .clicked()
                .then(|| settings_ui_state.set(ShowSettingsUiState::Inactive));
        });
    });
}
