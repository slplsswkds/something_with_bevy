use super::{UniversalCameraControllerBridge, UniversalCameraControllerTrait};
// use bevy::prelude::{Component, EulerRot, KeyCode, Quat, Vec3};
use bevy::prelude::*;

#[derive(Component)]
pub struct FlyingCamera {
    sensibility: f32,
    speed: f32,
    pitch: f32,
    yaw: f32,
}

impl Default for FlyingCamera {
    fn default() -> Self {
        Self {
            sensibility: 0.002,
            speed: 100.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl UniversalCameraControllerTrait for FlyingCamera {
    fn update(&mut self, bridge: &mut UniversalCameraControllerBridge) {
        self.update_position(bridge);
        self.update_view(bridge);
    }
}

impl FlyingCamera {
    fn update_position(&mut self, bridge: &mut UniversalCameraControllerBridge) {
        #[cfg(debug_assertions)]
        if bridge.cam_transform.is_empty() {
            warn!("FlyingCamera::update_position: no camera found")
        }

        for mut cam_transform in bridge.cam_transform.iter_mut() {
            let forward = (cam_transform.rotation * Vec3::Z).normalize();
            let right = (cam_transform.rotation * Vec3::X).normalize();

            let delta_move = self.speed * bridge.res_time.delta_secs();

            let mut desired_position = cam_transform.translation;

            if bridge.keys.pressed(KeyCode::KeyW) {
                desired_position -= forward * delta_move;
            }
            if bridge.keys.pressed(KeyCode::KeyS) {
                desired_position += forward * delta_move;
            }
            if bridge.keys.pressed(KeyCode::KeyD) {
                desired_position += right * delta_move;
            }
            if bridge.keys.pressed(KeyCode::KeyA) {
                desired_position -= right * delta_move;
            }
            if bridge.keys.pressed(KeyCode::Space) {
                desired_position.y += delta_move;
            }
            if bridge.keys.pressed(KeyCode::ControlLeft) {
                desired_position.y -= delta_move;
            }

            cam_transform.translation = cam_transform.translation.lerp(desired_position, 0.1);
        }
    }

    fn update_view(&mut self, bridge: &mut UniversalCameraControllerBridge) {
        let mut cam_transform = bridge.cam_transform.get_single_mut().unwrap();

        if self.pitch == 0.0 && self.yaw == 0.0 {
            let (yaw, pitch, _) = cam_transform.rotation.to_euler(EulerRot::YXZ);
            self.yaw = yaw + 1e-3;
            self.pitch = pitch + 1e-3;
        }

        for event in bridge.evr_mouse_movement.read() {
            self.yaw -= self.sensibility * event.delta.x;
            self.pitch -= self.sensibility * event.delta.y;

            self.pitch = self
                .pitch
                .clamp(-90.0_f32.to_radians(), 90.0_f32.to_radians());

            let yaw_rotation = Quat::from_rotation_y(self.yaw);
            let pitch_rotation = Quat::from_rotation_x(self.pitch);

            let desired_rotation = yaw_rotation * pitch_rotation;
            let interpolated_rotation = cam_transform.rotation.slerp(desired_rotation, 0.3);
            cam_transform.rotation = interpolated_rotation;
        }
    }
}
