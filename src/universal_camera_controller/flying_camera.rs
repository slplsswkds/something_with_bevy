use super::{Bridge, UniversalCameraTrait};
// use bevy::prelude::{Component, EulerRot, KeyCode, Quat, Vec3};
use bevy::prelude::*;

#[derive(Component)]
pub struct FlyingCamera {
    desired_position: Vec3,
    pitch: f32,
    yaw: f32,
}

impl Default for FlyingCamera {
    fn default() -> Self {
        Self {
            desired_position: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl UniversalCameraTrait for FlyingCamera {
    fn update(&mut self, bridge: &mut Bridge) {
        self.update_position(bridge);
        self.update_view(bridge);
    }
}

impl FlyingCamera {
    fn update_position(&mut self, bridge: &mut Bridge) {
        #[cfg(debug_assertions)]
        if bridge.cam_transform.is_empty() {
            warn!("FlyingCamera::update_position: no camera found")
        }

        let mut cam_transform = bridge.cam_transform.get_single_mut().unwrap();
        let forward = (cam_transform.rotation * Vec3::Z).normalize();
        let right = (cam_transform.rotation * Vec3::X).normalize();

        let delta_move = bridge.settings.movement_speed * bridge.time.delta_secs();

        let mut desired_position = if self.desired_position == Vec3::ZERO {
            cam_transform.translation
        } else {
            self.desired_position
        };

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

        self.desired_position = desired_position;

        // cam_transform.translation = cam_transform.translation.lerp(desired_position, 0.5);

        let t = 1.0 - (-20.0 * bridge.time.delta_secs()).exp();
        cam_transform.translation = cam_transform.translation.lerp(desired_position, t);
    }

    fn update_view(&mut self, bridge: &mut Bridge) {
        let mut cam_transform = bridge.cam_transform.get_single_mut().unwrap();

        if self.pitch == 0.0 && self.yaw == 0.0 {
            let (yaw, pitch, _) = cam_transform.rotation.to_euler(EulerRot::YXZ);
            self.yaw = yaw + 1e-3;
            self.pitch = pitch + 1e-3;
        }

        let mut total_delta_x = 0.0;
        let mut total_delta_y = 0.0;

        for event in bridge.evr_mouse_movement.read() {
            total_delta_x += event.delta.x;
            total_delta_y += event.delta.y;
        }

        self.yaw -= bridge.settings.sensitivity_horizontal * total_delta_x;
        self.pitch -= bridge.settings.sensitivity_vertical * total_delta_y;

        self.pitch = self
            .pitch
            .clamp(-90.0_f32.to_radians(), 90.0_f32.to_radians());

        let yaw_rotation = Quat::from_rotation_y(self.yaw);
        let pitch_rotation = Quat::from_rotation_x(self.pitch);

        let t = 1.0 - (-30.0 * bridge.time.delta_secs()).exp(); // Гладка інтерполяція
        cam_transform.rotation = cam_transform
            .rotation
            .slerp(yaw_rotation * pitch_rotation, t);
    }
}
