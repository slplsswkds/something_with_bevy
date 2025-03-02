use bevy::prelude::KeyCode;

#[derive(Clone)]
pub struct KeyboardBindings {
    // Movement (Universal Camera Controller)
    forward: KeyCode,
    backward: KeyCode,
    left: KeyCode,
    right: KeyCode,
    // Building
    start_building: KeyCode,
    stop_building: KeyCode,
}

impl Default for KeyboardBindings {
    fn default() -> Self {
        Self {
            // Movement (Universal Camera Controller)
            forward: KeyCode::KeyW,
            backward: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            // Building
            start_building: KeyCode::KeyB,
            stop_building: KeyCode::KeyN,
        }
    }
}
