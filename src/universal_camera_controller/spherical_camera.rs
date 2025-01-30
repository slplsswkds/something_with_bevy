use super::{UniversalCameraControllerBridge, UniversalCameraControllerTrait};
use bevy::prelude::{Component, KeyCode};

#[allow(dead_code)]
#[derive(Component)]
pub struct SphericalCamera {
    radius: f32,
    theta: f32,
    phi: f32,
}

impl Default for SphericalCamera {
    fn default() -> Self {
        Self {
            radius: 10.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

impl UniversalCameraControllerTrait for SphericalCamera {
    fn update(&mut self, bridge: &mut UniversalCameraControllerBridge) {
        todo!()
    }
}
