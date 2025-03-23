use super::*;
use bevy::prelude::*;

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
