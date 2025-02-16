use bevy::prelude::Resource;

#[derive(Resource)]
pub struct UniCamSettings {
    pub sensitivity_vertical: f32,
    pub sensitivity_horizontal: f32,
    pub movement_speed: f32,
}

impl Default for UniCamSettings {
    fn default() -> Self {
        Self {
            sensitivity_vertical: 0.001,
            sensitivity_horizontal: 0.0015,
            movement_speed: 7.0,
        }
    }
}