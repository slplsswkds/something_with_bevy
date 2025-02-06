mod flying_camera;
mod ray_casting;
mod spherical_camera;

use crate::universal_camera_controller::prelude::RayCaster;
use crate::universal_camera_controller::ray_casting::uni_cam_ray_cast;
use crate::FlyingCamera;
use crate::SphericalCamera;
use bevy::ecs::system::SystemParam;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::flying_camera::FlyingCamera;
    pub use super::ray_casting::RayCaster;
    pub use super::spherical_camera::SphericalCamera;
    pub use super::UniCamChangeStateEvent;
    pub use super::UniCamController;
    pub use super::UniCamPlugin;
    pub use super::UniCamSettings;
    pub use super::UniCamState;
}

#[derive(Resource)]
pub struct UniCamSettings {
    sensitivity_vertical: f32,
    sensitivity_horizontal: f32,
    movement_speed: f32,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniCamState {
    Disabled,
    #[default]
    Enabled,
}

impl Default for UniCamSettings {
    fn default() -> Self {
        Self {
            sensitivity_vertical: 0.001,
            sensitivity_horizontal: 0.0015,
            movement_speed: 7.0,
        }
    }
}

pub struct UniCamPlugin;

impl Plugin for UniCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UniCamState>()
            .add_event::<UniCamChangeStateEvent>()
            .init_resource::<RayCaster>()
            .init_resource::<UniCamSettings>()
            .add_systems(Update, uni_cam_watchdog)
            .add_systems(
                Update,
                (change_cam_mode, uni_cam_controller, uni_cam_ray_cast)
                    .run_if(in_state(UniCamState::Enabled)),
            );
    }
}

#[derive(Event)]
pub struct UniCamChangeStateEvent(pub UniCamState);

fn uni_cam_watchdog(
    mut events: EventReader<UniCamChangeStateEvent>,
    mut controller_state: ResMut<NextState<UniCamState>>,
) {
    events.read().for_each(|event| {
        controller_state.set(event.0.clone());
    })
}

fn uni_cam_controller(mut universal_camera: Query<&mut UniCamController>, mut bridge: Bridge) {
    universal_camera.iter_mut().for_each(|mut cam_controller| {
        cam_controller.mode.update(&mut bridge);
    })
}

// maybe i should use events?
fn change_cam_mode(
    mut universal_camera: Query<&mut UniCamController>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    universal_camera.iter_mut().for_each(|mut cam_controller| {
        if keys.just_pressed(KeyCode::F1) {
            *cam_controller = UniCamController::flying_camera();
        }
        if keys.just_pressed(KeyCode::F3) {
            *cam_controller = UniCamController::spherical_camera();
        }
    })
}

#[derive(SystemParam)]
struct Bridge<'w, 's> {
    time: Res<'w, Time>,
    settings: Res<'w, UniCamSettings>,
    cam_transform: Query<'w, 's, &'static mut Transform, With<UniCamController>>,
    evr_mouse_movement: EventReader<'w, 's, MouseMotion>,
    keys: Res<'w, ButtonInput<KeyCode>>,
}

trait UniversalCameraTrait: Send + Sync {
    fn update(&mut self, bridge: &mut Bridge);
}

#[derive(Component)]
pub struct UniCamController {
    mode: Box<dyn UniversalCameraTrait>,
}

#[allow(dead_code)]
impl UniCamController {
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
