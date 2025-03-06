use super::keyboard::KeyboardBindings;
use super::mouse::MouseSensitivity;
use super::video::VideoSettings;
use super::GameSettingsBridge;
use bevy::log::warn;
use bevy::prelude::Resource;
use bevy::window::WindowResolution;

#[derive(Resource, Default, Clone)]
pub struct GameSettings {
    pub keyboard: KeyboardBindings,
    pub mouse: MouseSensitivity,
    pub video: VideoSettings,
}

impl GameSettings {
    pub fn try_load_from_disk() -> Option<GameSettings> {
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
