#[derive(Clone)]
pub struct MouseSensitivity {
    pub sensitivity_vertical: f32,
    pub sensitivity_horizontal: f32,
}

impl Default for MouseSensitivity {
    fn default() -> Self {
        Self {
            sensitivity_vertical: 0.001,
            sensitivity_horizontal: 0.0015,
        }
    }
}
