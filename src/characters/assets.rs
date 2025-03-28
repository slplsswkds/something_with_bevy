use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AssetsReadinessState {
    #[default]
    Loading,
    Ready,
}

#[derive(AssetCollection, Resource)]
pub struct CharactersAssets {
    #[asset(path = "characters/soldat/scene.gltf#Scene0")]
    pub soldat: Handle<Scene>,
    // #[asset(path = "characters/warrior_idle.glb#Animation0")]
    // pub animation: Handle<AnimationClip>,
}

/*
toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf srgb --assign_oetf srgb --zcmp 20 color.ktx2 color.png
toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf linear --assign_oetf linear --zcmp 20 metallicRoughness.ktx2 metallicRoughness.png
toktx --t2 --genmipmap --encode uastc --uastc_quality 3 --filter lanczos4 --convert_oetf linear --assign_oetf linear --zcmp 20 --normal_mode normal.ktx2 normal.png
 */
