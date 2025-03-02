mod main_menu;
mod settings_menu;

use bevy::prelude::*;
use main_menu::{enter_main_menu, exit_main_menu, main_menu};
use settings_menu::{enter_settings_ui, exit_settings_ui, settings_ui};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum MainMenuActivityState {
    #[default]
    Inactive,
    Active,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum ShowSettingsUiState {
    #[default]
    Inactive,
    Active,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuActivityState>();
        app.init_state::<ShowSettingsUiState>();
        app.add_systems(Update, watchdog);
        app.add_systems(OnEnter(MainMenuActivityState::Active), enter_main_menu);
        app.add_systems(
            Update,
            main_menu.run_if(in_state(MainMenuActivityState::Active)),
        );
        app.add_systems(
            OnEnter(ShowSettingsUiState::Active),
            enter_settings_ui.run_if(in_state(MainMenuActivityState::Active)),
        );
        app.add_systems(
            Update,
            settings_ui.run_if(
                in_state(MainMenuActivityState::Active).and(in_state(ShowSettingsUiState::Active)),
            ),
        );
        app.add_systems(
            OnExit(ShowSettingsUiState::Active),
            exit_settings_ui.run_if(in_state(MainMenuActivityState::Active)),
        );
        app.add_systems(OnExit(MainMenuActivityState::Active), exit_main_menu);
    }
}

fn watchdog(
    mut current_state: ResMut<State<MainMenuActivityState>>,
    mut next_state: ResMut<NextState<MainMenuActivityState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            MainMenuActivityState::Inactive => {
                next_state.set(MainMenuActivityState::Active);
            }
            MainMenuActivityState::Active => {
                next_state.set(MainMenuActivityState::Inactive);
            }
        }
    }
}
