use super::{UniversalCameraControllerBridge, UniversalCameraControllerTrait};
use bevy::math::Vec3;
use bevy::prelude::{Component, KeyCode};
use std::f32::consts::PI;

#[allow(dead_code)]
#[derive(Component)]
pub struct SphericalCamera {
    sensibility_vertical: f32,
    sensibility_horizontal: f32,
    radius: f32,
    theta: f32,
    phi: f32,
}

impl Default for SphericalCamera {
    fn default() -> Self {
        Self {
            sensibility_vertical: 0.003,
            sensibility_horizontal: 0.004,
            radius: 10.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

impl UniversalCameraControllerTrait for SphericalCamera {
    fn update(&mut self, bridge: &mut UniversalCameraControllerBridge) {
        for event in bridge.evr_mouse_movement.read() {
            self.phi -= self.sensibility_horizontal * event.delta.x;
            self.theta += self.sensibility_vertical * event.delta.y;

            // self.phi = self.phi.clamp(-181_f32.to_radians(), 181_f32.to_radians());
            self.theta = self.theta.clamp(10_f32.to_radians(), 89.9_f32.to_radians());

            let x = self.radius * self.theta.cos() * self.phi.sin();
            let y = self.radius * self.theta.sin();
            let z = self.radius * self.theta.cos() * self.phi.cos();

            let new_position = Vec3::new(x, y, z);

            bridge
                .cam_transform
                .iter_mut()
                .for_each(|mut cam_transform| {
                    cam_transform.translation = new_position;
                    cam_transform.look_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y); // target
                })
        }
    }
}
