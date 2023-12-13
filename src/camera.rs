//! Soochcart camera stuff.

use bevy::prelude::*;

/// A camera plugin.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, follow_kart_camera.before(CameraSystem::Orbit))
            .add_systems(Update, orbit_camera.in_set(CameraSystem::Orbit));
    }
}

/// Camera systems.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum CameraSystem {
    /// Translates the camera in the orbit component.
    Orbit,
}

/// A camera that orbits a subject.
#[derive(Clone, Component, Debug)]
pub struct Orbit {
    /// The subject to orbit.
    pub subject: Option<Entity>,
    /// The distance to orbit.
    pub distance: f32,
    /// The rotation of the orbit.
    pub rot: Quat,
}

impl Default for Orbit {
    fn default() -> Orbit {
        Orbit {
            subject: None,
            distance: 5.,
            rot: Quat::IDENTITY,
        }
    }
}

/// A bundle for a camera that follows a kart.
#[derive(Bundle)]
pub struct FollowKartBundle {
    follow_kart: FollowKart,
    orbit: Orbit,
}

impl FollowKartBundle {
    pub fn new(kart: Entity) -> FollowKartBundle {
        FollowKartBundle {
            follow_kart: FollowKart {
                kart: Some(kart),
                ..default()
            },
            orbit: Orbit {
                subject: Some(kart),
                ..default()
            },
        }
    }
}

/// A camera that follows a kart with rotational smoothing.
#[derive(Clone, Component, Debug)]
pub struct FollowKart {
    /// The kart to follow.
    pub kart: Option<Entity>,
    yaw: f32,
}

impl FollowKart {
    /// Returns the orientation of the camera.
    pub fn rotation(&self) -> Quat {
        Quat::from_axis_angle(Vec3::X, (-15f32).to_radians())
            * Quat::from_axis_angle(Vec3::Y, self.yaw)
    }
}

impl Default for FollowKart {
    fn default() -> FollowKart {
        FollowKart {
            kart: None,
            yaw: 0.,
        }
    }
}

fn follow_kart_camera(
    mut camera_query: Query<(&mut Orbit, &FollowKart)>,
    _kart_query: Query<&GlobalTransform>,
) {
    for (mut orbit, follow_kart) in camera_query.iter_mut() {
        orbit.rot = follow_kart.rotation();
    }
}

fn orbit_camera(
    mut camera_query: Query<(&mut Transform, &Orbit)>,
    transform_query: Query<&GlobalTransform>,
) {
    for (mut transform, orbit) in camera_query.iter_mut() {
        let Some(subject_transform) = orbit.subject.and_then(|s| transform_query.get(s).ok())
        else {
            continue;
        };

        let forward = orbit.rot * Vec3::Z;

        // start with subject position
        *transform = Transform::from_translation(subject_transform.translation())
            // back camera away
            * Transform::from_translation(forward * orbit.distance)
            // face camera in direction
            * Transform::from_rotation(orbit.rot);
    }
}
