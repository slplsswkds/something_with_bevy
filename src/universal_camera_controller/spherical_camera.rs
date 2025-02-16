use super::{Bridge, UniCamTrait};
use bevy::math::{Mat3, Quat};
use bevy::prelude::{Component, Transform, Vec3};

#[allow(dead_code)]
#[derive(Component)]
pub struct SphericalCamera {
    desired_position: Vec3,
    radius: f32,
    theta: f32,
    phi: f32,
}

impl Default for SphericalCamera {
    fn default() -> Self {
        Self {
            desired_position: Vec3::ZERO,
            radius: 3.0,
            theta: 0.0,
            phi: 0.0,
        }
    }
}

impl UniCamTrait for SphericalCamera {
    fn update(&mut self, bridge: &mut Bridge) {
        let mut total_delta_x = 0.0;
        let mut total_delta_y = 0.0;

        // Reading mouse movement
        for event in bridge.evr_mouse_movement.read() {
            total_delta_x += event.delta.x;
            total_delta_y += event.delta.y;
        }

        // Calculating camera rotation
        self.phi -= bridge.settings.sensitivity_horizontal * total_delta_x;
        self.theta += bridge.settings.sensitivity_vertical * total_delta_y;
        self.theta = self.theta.clamp(10_f32.to_radians(), 89.9_f32.to_radians());

        // Calculating new camera position
        let x = self.radius * self.theta.cos() * self.phi.sin();
        let y = self.radius * self.theta.sin();
        let z = self.radius * self.theta.cos() * self.phi.cos();
        let new_position = Vec3::new(x, y, z);

        // Direction to look at the target (Vec3::ZERO)
        let look_at_direction = (new_position - Vec3::ZERO).normalize(); // Inverted direction to make the camera look at the target
        let up = Vec3::Y; // World up vector

        // Calculate right and up vectors
        let right = up.cross(look_at_direction).normalize();
        let up_corrected = look_at_direction.cross(right); // Correct up vector to avoid gimbal lock

        // Desired rotation to face the target (look-at behavior)
        let desired_rotation =
            Quat::from_mat3(&Mat3::from_cols(right, up_corrected, look_at_direction));

        // Smooth interpolation factor based on delta time
        let t = 1.0 - (-30.0 * bridge.time.delta_secs()).exp();

        // Apply the transformations to the camera
        let cam_transform: &mut Transform = &mut *bridge.cam_transform;

        // Smoothly interpolate position
        cam_transform.translation = cam_transform.translation.lerp(new_position, t);

        // Smoothly interpolate rotation to make the camera face the target
        cam_transform.rotation = cam_transform.rotation.slerp(desired_rotation, t);
    }
}
