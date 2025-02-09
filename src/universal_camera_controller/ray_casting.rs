use super::UniCamController;
use bevy::prelude::*;

/// Contains the entity the UniversalCameraController is looking at
#[derive(Resource)]
pub struct RayCaster(Option<Entity>);

impl Default for RayCaster {
    fn default() -> Self {
        Self(None)
    }
}

pub fn uni_cam_ray_cast(
    camera_query: Query<&GlobalTransform, With<UniCamController>>,
    mut ray_cast: MeshRayCast,
    ray_caster: Res<RayCaster>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let origin = camera_transform.translation();
        let direction = camera_transform.forward();

        let ray = Ray3d::new(origin, direction);

        let settings = RayCastSettings::default();

        let hits = ray_cast.cast_ray(ray, &settings);

        for (entity, hit) in hits {
            // println!(
            //     "The ray hit the object {:?} at point {:?}",
            //     entity, hit.point
            // );
        }
    }
}
