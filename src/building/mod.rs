mod building;
mod assets;
mod menu;
mod plugin;

use bevy::prelude::*;
use building::prelude::*;
use assets::{BuildingAssets, BuildingAssetsInitBridge, PreviewBuildingHandle};
use menu::{building_menu, enter_building_menu, exit_building_menu};

pub use plugin::BuildingPlugin;

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
