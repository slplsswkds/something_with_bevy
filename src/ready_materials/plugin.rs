use super::systems::*;
use super::*;
use bevy::prelude::{App, Plugin};

pub struct ReadyMaterialsPlugin;

impl Plugin for ReadyMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ReadyMaterialsState>();

        app.add_systems(OnEnter(ReadyMaterialsState::NotReady), init_materials);

        app.add_systems(
            Update,
            track_load_progress.run_if(in_state(ReadyMaterialsState::Loading)),
        );

        app.add_systems(OnEnter(ReadyMaterialsState::Ready), init_materials);

        // app.insert_resource();
    }
}
