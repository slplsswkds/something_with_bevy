use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UniCamSettings {
    pub movement_speed: f32,
}

impl Default for UniCamSettings {
    fn default() -> Self {
        Self {
            movement_speed: 7.0,
        }
    }
}