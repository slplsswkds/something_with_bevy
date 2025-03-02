use bevy::window::{MonitorSelection, PresentMode, Window, WindowMode, WindowResolution};

#[derive(Clone)]
pub struct VideoSettings {
    // Window
    /// Which fullscreen or windowing mode should be used.
    pub window_mode: WindowMode,
    pub window_width: u32,
    pub window_height: u32,
    pub present_mode: PresentMode,
    // Graphics
}

impl Default for VideoSettings {
    fn default() -> Self {
        Self {
            window_mode: WindowMode::Windowed,
            window_width: 1920,
            window_height: 1200,
            present_mode: PresentMode::AutoVsync,
        }
    }
}
