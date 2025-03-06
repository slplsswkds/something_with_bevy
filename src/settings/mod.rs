mod game_settings;
mod keyboard;
mod mouse;
mod plugin;
mod video;

use bevy::ecs::system::SystemParam;
use bevy::prelude::{Single, Window};

pub use game_settings::GameSettings;
pub use plugin::GameSettingsPlugin;

#[derive(SystemParam)]
pub struct GameSettingsBridge<'w> {
    window: Single<'w, &'static mut Window>,
}
