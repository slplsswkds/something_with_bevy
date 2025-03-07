mod flying_camera;
mod plugin;
mod settings;
mod spherical_camera;

use crate::settings::GameSettings;
use bevy::ecs::system::SystemParam;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
pub use flying_camera::FlyingCamera;
pub use plugin::UniCamPlugin;
pub use settings::UniCamSettings;
pub use spherical_camera::SphericalCamera;

/// Represents the state of the camera controller (enabled/disabled).
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniCamState {
    Disabled,
    #[default]
    Enabled,
}

/// Event to change the camera controller state.
#[derive(Event)]
pub struct UniCamChangeStateEvent(pub UniCamState);

/// Monitors and updates the camera controller state.
fn uni_cam_watchdog(
    mut events: EventReader<UniCamChangeStateEvent>,
    mut controller_state: ResMut<NextState<UniCamState>>,
) {
    events.read().for_each(|new_state| {
        controller_state.set(new_state.0.clone());
    });
}

/// Updates the currently active camera.
fn uni_cam_controller(mut cam_controller: Single<&mut UniCamController>, mut bridge: Bridge) {
    cam_controller.mode.update(&mut bridge);
}

/// Switches between different camera modes when keys are pressed.
fn change_cam_mode(
    mut cam_controller: Single<&mut UniCamController>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F1) {
        **cam_controller = FlyingCamera::default().into();
    }
    if keys.just_pressed(KeyCode::F3) {
        **cam_controller = SphericalCamera::default().into();
    }
}

/// A system parameter that provides access to various resources
/// needed for camera updates, including time, settings,
/// camera transform, mouse movement events, and keyboard input.
/// This acts as a bridge between the camera controllers and the Bevy ECS.
#[derive(SystemParam)]
struct Bridge<'w, 's> {
    time: Res<'w, Time>,
    settings: Res<'w, UniCamSettings>,
    cam_transform: Single<'w, &'static mut Transform, With<UniCamController>>,
    evr_mouse_movement: EventReader<'w, 's, MouseMotion>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    game_settings: Res<'w, GameSettings>,
}

/// Trait that defines the behavior of different camera types.
trait UniCamTrait: Send + Sync {
    fn update(&mut self, bridge: &mut Bridge);
}

/// Main camera controller that holds the active camera mode.
#[derive(Component)]
pub struct UniCamController {
    mode: Box<dyn UniCamTrait>,
}

impl<T: UniCamTrait + 'static> From<T> for UniCamController {
    fn from(camera: T) -> Self {
        Self {
            mode: Box::new(camera),
        }
    }
}
