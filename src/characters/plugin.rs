use super::assets::{AssetsReadinessState, CharactersAssets};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetsReadinessState>()
            .add_loading_state(
                LoadingState::new(AssetsReadinessState::Loading)
                    .continue_to_state(AssetsReadinessState::Ready)
                    .load_collection::<CharactersAssets>(),
            )
            .add_systems(OnEnter(AssetsReadinessState::Ready), spawn_soldat);
    }
}

fn spawn_soldat(mut commands: Commands, characters_assets: Res<CharactersAssets>) {
    commands.spawn((
        SceneRoot(characters_assets.soldat.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(0.69)),
        AnimationPlayer::default(),
    ));
}
