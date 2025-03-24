mod bridge;
mod plugin;
mod systems;
mod resources;

use bevy::prelude::*;
pub use plugin::ReadyMaterialsPlugin;
pub use resources::ReadyMaterialsResource;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReadyMaterialsState {
    #[default]
    NotReady,
    Loading,
    Ready,
}
