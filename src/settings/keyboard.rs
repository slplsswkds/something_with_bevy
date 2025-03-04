use bevy::prelude::KeyCode;

#[derive(Clone)]
pub struct KeyboardBindings {
    // Movement (Universal Camera Controller)
    pub forward: KeyCode,
    pub backward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub crouch: KeyCode,
    // Building
    pub start_building: KeyCode,
    pub stop_building: KeyCode,
}

impl Default for KeyboardBindings {
    fn default() -> Self {
        Self {
            // Movement (Universal Camera Controller)
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            jump: KeyCode::Space,
            crouch: KeyCode::ControlLeft,
            // Building
            start_building: KeyCode::KeyB,
            stop_building: KeyCode::KeyN,
        }
    }
}
