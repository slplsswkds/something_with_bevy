mod building;
mod building_assets;
mod building_menu;

use bevy::prelude::*;
use building::prelude::*;
use building_assets::{BuildingAssets, BuildingAssetsInitBridge, PreviewBuildingHandle};
use building_menu::{building_menu, enter_building_menu, exit_building_menu};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum BuildingReadinessState {
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
        Self { grid_size: 0.1 }
    }
}

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BuildingReadinessState>()
            .init_state::<BuildingMode>()
            .init_resource::<BuildingSettings>()
            .init_resource::<PreviewBuildingHandle>()
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
            .add_systems(OnEnter(BuildingMode::Building), enter_building_mode)
            .add_systems(
                Update,
                (building_system, update_preview_building_position)
                    .chain()
                    .run_if(in_state(BuildingMode::Building)),
            )
            .add_systems(OnExit(BuildingMode::Building), exit_building_mode);
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
