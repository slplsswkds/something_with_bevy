use super::GameSettings;
use bevy::app::{App, Plugin};

pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        let game_settings = GameSettings::try_load_from_disk().unwrap_or_default();
        app.insert_resource(game_settings.clone());
    }
}
