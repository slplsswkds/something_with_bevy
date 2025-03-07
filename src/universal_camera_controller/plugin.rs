use super::{
    change_cam_mode, uni_cam_controller, uni_cam_watchdog, UniCamChangeStateEvent, UniCamSettings,
    UniCamState,
};
use bevy::prelude::{in_state, App, AppExtStates, IntoSystemConfigs, Plugin, Update};

pub struct UniCamPlugin;

impl Plugin for UniCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UniCamState>()
            .add_event::<UniCamChangeStateEvent>()
            .init_resource::<UniCamSettings>()
            .add_systems(Update, uni_cam_watchdog)
            .add_systems(
                Update,
                (change_cam_mode, uni_cam_controller).run_if(in_state(UniCamState::Enabled)),
            );
    }
}
