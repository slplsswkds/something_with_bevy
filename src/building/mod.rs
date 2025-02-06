mod building_assets;
mod building_menu;

use super::UniversalCameraController;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiContexts};
use building_assets::{BuildingAssets, BuildingAssetsInitBridge};
use building_menu::{building_menu, enter_building_menu, exit_building_menu};

#[allow(unused_imports)]
pub mod prelude {
    pub use super::BuildingMode;
    pub use super::BuildingPlugin;
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum BuildingReadinessState {
    NotReady,
    #[default]
    Loading,
    Ready,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuildingMode {
    #[default]
    Disabled,
    Menu,
    Building,
}

#[derive(Resource)]
struct BuildingSettings {
    grid_size: f32,
}

impl Default for BuildingSettings {
    fn default() -> Self {
        Self { grid_size: 1.0 }
    }
}

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BuildingReadinessState>()
            .init_state::<BuildingMode>()
            .init_resource::<BuildingSettings>()
            .add_event::<ChangeBuildingModeEvent>()
            .add_systems(
                OnEnter(BuildingReadinessState::Loading),
                load_building_assets,
            )
            .add_systems(
                Update,
                building_watchdog_system.run_if(in_state(BuildingReadinessState::Ready)),
            )
            // ---------- Menu Mode
            .add_systems(OnEnter(BuildingMode::Menu), enter_building_menu)
            .add_systems(Update, building_menu.run_if(in_state(BuildingMode::Menu)))
            .add_systems(OnExit(BuildingMode::Menu), exit_building_menu)
            // ---------- Building Mode
            .add_systems(OnEnter(BuildingMode::Building), init_building_mode)
            .add_systems(
                Update,
                (building_system, update_preview_building_position)
                    .chain()
                    .run_if(in_state(BuildingMode::Building)),
            )
            .add_systems(OnExit(BuildingMode::Building), deinit_building_mode);
    }
}

/// Load all assets and switch BuildingMode state to Disabled
/// Sets BuildingReadinessState::Ready when finished
fn load_building_assets(
    mut commands: Commands,
    mut building_readiness_state: ResMut<NextState<BuildingReadinessState>>,
    bridge: BuildingAssetsInitBridge,
) {
    commands.insert_resource(BuildingAssets::load_all(bridge));
    // wait while resources loading !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    building_readiness_state.set(BuildingReadinessState::Ready);
    info!("BuildingReadinessState::Ready");
}

// ---------- Building Mode
fn init_building_mode(mut commands: Commands, assets: Res<BuildingAssets>) {
    // show UI with building menu
    // let preview: Handle<Scene> = assets.wall.wall_2x2.clone();
    if let Some(preview) = assets.preview_obj.clone() {
        commands.spawn((SceneRoot(preview), PreviewBuilding));
    } else {
        error!("No preview_obj found in assets.preview_obj. Does it okay?");
    }
}

fn building_system(
    mut commands: Commands,
    // mut preview_building: Query<&Transform, With<PreviewBuilding>>,
    mut preview_building: Query<(&SceneRoot, &Transform), With<PreviewBuilding>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let (root, transform) = preview_building.single();
    if buttons.just_pressed(MouseButton::Left) {
        commands.spawn((root.clone(), transform.clone()));
    }
}

/// Moves the building preview to the correct position relative to the camera and the global grid
fn update_preview_building_position(
    mut params: ParamSet<(
        Query<&mut Transform, With<PreviewBuilding>>,
        Query<&Transform, With<UniversalCameraController>>,
    )>,
    building_settings: Res<BuildingSettings>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let cam_transform = params.p1().get_single().unwrap().clone();

    let mut vertical_scroll = 0_f32;
    evr_scroll
        .read()
        .enumerate()
        .for_each(|(idx, scroll)| match scroll.unit {
            MouseScrollUnit::Line => {
                vertical_scroll += scroll.y;
            }
            MouseScrollUnit::Pixel => {}
        });

    params.p0().iter_mut().for_each(|mut transform| {
        transform
            .translation
            .round_to_step(building_settings.grid_size);

        let rotation = Quat::from_rotation_y(vertical_scroll * 15_f32.to_radians());
        transform.rotation *= rotation;

        let distance_in_front = 7.0;
        let camera_position = cam_transform.translation;
        let camera_forward = cam_transform.rotation * Vec3::NEG_Z;

        let new_cube_position = camera_position + camera_forward * distance_in_front;
        transform.translation = new_cube_position.round_to_step(building_settings.grid_size);
    });
}

fn deinit_building_mode(
    mut commands: Commands,
    preview_building_query: Query<Entity, With<PreviewBuilding>>,
) {
    preview_building_query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
// ------------------------------

#[derive(Event)]
struct ChangeBuildingModeEvent(BuildingMode);

/// Based on user actions, switches the desired construction mode.
fn building_watchdog_system(
    mut ev_switch_mode: EventReader<ChangeBuildingModeEvent>,
    mut building_mode_state: ResMut<NextState<BuildingMode>>,
    keys: Res<ButtonInput<KeyCode>>, // for debug only !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
) {
    for ev in ev_switch_mode.read() {
        info!("Changing building mode: {:?}", ev.0);
        match ev.0 {
            BuildingMode::Menu => building_mode_state.set(BuildingMode::Menu),
            BuildingMode::Building => building_mode_state.set(BuildingMode::Building),
            BuildingMode::Disabled => building_mode_state.set(BuildingMode::Disabled),
        }
    }

    // for debug only !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    if keys.just_pressed(KeyCode::KeyB) {
        building_mode_state.set(BuildingMode::Menu);
        info!("Changing building mode: Building");
    } else if keys.just_pressed(KeyCode::KeyN) {
        building_mode_state.set(BuildingMode::Disabled);
        info!("Changing building mode: Disabled");
    }
}

#[derive(Component)]
struct PreviewBuilding;

trait RoundToStep {
    fn round_to_step(self, step: f32) -> Self;
}

impl RoundToStep for Vec3 {
    fn round_to_step(self, step: f32) -> Self {
        (self / step).round() * step
    }
}
