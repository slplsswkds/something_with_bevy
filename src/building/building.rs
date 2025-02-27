use super::building_assets::BuildingAssets;
use super::{BuildingSettings, PreviewBuilding, RoundToStep};
use crate::universal_camera_controller::UniCamController;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub mod prelude {
    pub use super::building_system;
    pub use super::enter_building_mode;
    pub use super::exit_building_mode;
    pub use super::update_preview_building_position;
}

/// Initializes the building mode by spawning the preview object.
pub fn enter_building_mode(mut commands: Commands, assets: Res<BuildingAssets>) {
    if let Some(preview) = assets.preview_obj.clone() {
        commands.spawn((SceneRoot(preview), PreviewBuilding));
    } else {
        error!("No preview_obj found in assets.preview_obj. Does it okay?");
    }
}

/// Handles the building system by placing a building when the left mouse button is pressed.
pub fn building_system(
    mut commands: Commands,
    mut preview_building: Query<(&SceneRoot, &Transform), With<PreviewBuilding>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some((root, transform)) = preview_building.iter().next() {
            commands.spawn((root.clone(), transform.clone()));
        }
    }
}

/// Updates the position of the building preview relative to the camera and grid.
pub fn update_preview_building_position(
    mut params: ParamSet<(
        Single<&mut Transform, With<PreviewBuilding>>,
        Single<&Transform, With<UniCamController>>,
    )>,
    building_settings: Res<BuildingSettings>,
    mut evr_scroll: EventReader<MouseWheel>,
) {
    let mut vertical_scroll = 0_f32;
    evr_scroll.read().for_each(|scroll| match scroll.unit {
        MouseScrollUnit::Line => {
            vertical_scroll += scroll.y;
        }
        MouseScrollUnit::Pixel => {}
    });

    let cam_transform = params.p1().clone();
    let mut building_transform = params.p0();

    building_transform
        .translation
        .round_to_step(building_settings.grid_size);

    let rotation = Quat::from_rotation_y(vertical_scroll * 15_f32.to_radians());
    building_transform.rotation *= rotation;

    let distance_in_front = 7.0;
    let camera_position = cam_transform.translation;
    let camera_forward = cam_transform.rotation * Vec3::NEG_Z;

    let new_cube_position = camera_position + camera_forward * distance_in_front;
    building_transform.translation = new_cube_position.round_to_step(building_settings.grid_size);
}

///Destroy the preview building entity.
pub fn exit_building_mode(
    mut commands: Commands,
    preview_building_query: Query<Entity, With<PreviewBuilding>>,
) {
    preview_building_query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
