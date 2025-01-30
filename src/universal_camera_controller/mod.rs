mod flying_camera;
mod spherical_camera;

use crate::FlyingCamera;
use crate::SphericalCamera;
use bevy::ecs::system::SystemParam;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub mod prelude {
    pub use super::flying_camera::FlyingCamera;
    pub use super::spherical_camera::SphericalCamera;
    pub use super::UniversalCameraController;
    pub use super::UniversalCameraControllerPlugin;
}

pub struct UniversalCameraControllerPlugin;

impl Plugin for UniversalCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, universal_camera_controller_system);
    }
}

fn universal_camera_controller_system(
    mut universal_camera: Query<&mut UniversalCameraController>,
    mut bridge: UniversalCameraControllerBridge,
) {
    universal_camera.iter_mut().for_each(|mut camera| {
        camera.mode.update(&mut bridge);
    })
}

#[derive(SystemParam)]
struct UniversalCameraControllerBridge<'w, 's> {
    res_time: Res<'w, Time>,
    cam_transform: Query<'w, 's, &'static mut Transform, With<Camera3d>>,
    evr_mouse_movement: EventReader<'w, 's, MouseMotion>,
    keys: Res<'w, ButtonInput<KeyCode>>,
}

trait UniversalCameraControllerTrait: Send + Sync {
    fn update(&mut self, bridge: &mut UniversalCameraControllerBridge);
}

#[derive(Component)]
pub struct UniversalCameraController {
    mode: Box<dyn UniversalCameraControllerTrait>,
}

impl UniversalCameraController {
    pub fn spherical_camera() -> Self {
        Self {
            mode: Box::new(SphericalCamera::default()),
        }
    }
    pub fn flying_camera() -> Self {
        Self {
            mode: Box::new(FlyingCamera::default()),
        }
    }
}
