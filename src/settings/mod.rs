use super::settings::keyboard::KeyboardBindings;
use super::settings::mouse::MouseSensitivity;
use super::settings::video::VideoSettings;
use bevy::app::{App, Plugin, Startup};
use bevy::ecs::system::SystemParam;
use bevy::log::warn;
use bevy::prelude::{Res, Resource, Single, Window};
use bevy::window::WindowResolution;

mod keyboard;
mod mouse;
mod video;

pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        let game_settings = GameSettings::try_load_from_disk().unwrap_or_default();
        app.insert_resource(game_settings.clone());
        app.add_systems(Startup, apply_settings);
    }
}

fn apply_settings(settings: Res<GameSettings>, mut bridge: GameSettingsBridge) {
    settings.apply_settings(&mut bridge)
}

#[derive(Resource, Default, Clone)]
pub struct GameSettings {
    pub keyboard: KeyboardBindings,
    pub mouse: MouseSensitivity,
    pub video: VideoSettings,
}

#[derive(SystemParam)]
pub struct GameSettingsBridge<'w> {
    window: Single<'w, &'static mut Window>,
}

impl GameSettings {
    fn try_load_from_disk() -> Option<GameSettings> {
        warn!("Loading game settings from disk currently not supported");
        None
    }

    pub fn apply_settings(
        &self,
        bridge: &mut GameSettingsBridge,
        // mut window: Single<&mut Window>,
    ) {
        bridge.window.resolution = WindowResolution::new(
            self.video.window_width as f32,
            self.video.window_height as f32,
        );
        bridge.window.mode = self.video.window_mode;
        bridge.window.present_mode = self.video.present_mode;
    }
}
