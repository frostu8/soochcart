//! A wheel and its suspension.

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use super::KartSystem;

/// Wheel plugin.
pub struct WheelPlugin;

impl Plugin for WheelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Wheel>()
            .add_systems(FixedUpdate, do_wheel_raycast.in_set(WheelSystem::Raycast))
            .add_systems(
                FixedUpdate,
                apply_wheel_transform.after(WheelSystem::Raycast),
            )
            .add_systems(
                FixedUpdate,
                apply_wheel_forces
                    .in_set(WheelSystem::ApplyForce)
                    .after(KartSystem::ResetForces)
                    .after(WheelSystem::Raycast),
            );
    }
}

/// A system set for wheels.
#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum WheelSystem {
    /// Does the raycast.
    Raycast,
    /// Applies forces.
    ApplyForce,
}

/// A wheel bundle.
#[derive(Bundle, Default)]
pub struct WheelBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub wheel: Wheel,
}

impl WheelBundle {
    /// Creates a new [`WheelBundle`] at a position.
    pub fn new(position: Vec3) -> WheelBundle {
        WheelBundle {
            wheel: Wheel::new(position, 0.4),
            ..default()
        }
    }
}

/// A single wheel.
///
/// The position will be determined by the raycast.
#[derive(Clone, Component, Debug, Reflect)]
pub struct Wheel {
    /// The position of the suspension, relative to the parent.
    pub position: Vec3,
    /// The max suspension length in meters.
    pub max_suspension: f32,
    /// The force that is applied to the chassis, based on the extension ratio.
    pub max_force: f32,
    /// The damping force applied.
    pub damping_factor: f32,
    ratio: f32,
    normal: Option<Vec3>,
}

impl Wheel {
    /// Creates a new wheel.
    pub fn new(position: Vec3, max_suspension: f32) -> Wheel {
        Wheel {
            position,
            max_suspension,
            ..default()
        }
    }

    /// Checks if the suspension is fully extended.
    pub fn extended(&self) -> bool {
        self.ratio < f32::EPSILON
    }

    /// The ratio of the suspension.
    ///
    /// `0` is fully extended.
    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    /// The inverse ratio of the suspension.
    ///
    /// `1` is fully extended.
    pub fn ratio_minus_one(&self) -> f32 {
        1. - self.ratio
    }

    /// The normal of the suspension's contact point.
    pub fn normal(&self) -> Option<Vec3> {
        self.normal
    }
}

impl Default for Wheel {
    fn default() -> Wheel {
        Wheel {
            position: default(),
            max_suspension: 0.35,
            max_force: 4.,
            damping_factor: 0.2,
            ratio: 0.,
            normal: None,
        }
    }
}

fn apply_wheel_forces(
    mut chassis_query: Query<(
        &mut ExternalForce,
        &GlobalTransform,
        &Velocity,
        &ReadMassProperties,
    )>,
    wheel_query: Query<(&Parent, &Wheel)>,
) {
    for (chassis, wheel) in wheel_query.iter() {
        let Ok((mut ef, transform, velocity, mass_properties)) =
            chassis_query.get_mut(chassis.get())
        else {
            continue;
        };

        let mass_properties = mass_properties.get();

        let up = transform.up();

        let position = transform.transform_point(wheel.position);
        let center_of_mass = transform.transform_point(mass_properties.local_center_of_mass);

        // calculate impulse
        let pointvel = velocity.linear_velocity_at_point(position, center_of_mass);
        let damping = wheel.damping_factor * pointvel.dot(up);

        if !wheel.extended() {
            let force =
                wheel.max_force * mass_properties.mass * up * (wheel.ratio - damping).max(0.);

            *ef += ExternalForce::at_point(force, position, center_of_mass);
        }
    }
}

fn apply_wheel_transform(mut wheel_query: Query<(&mut Transform, &Wheel)>) {
    for (mut transform, wheel) in wheel_query.iter_mut() {
        let position = wheel.position + -Vec3::Y * wheel.max_suspension * wheel.ratio_minus_one();

        *transform = Transform::from_translation(position);
    }
}

fn do_wheel_raycast(
    chassis_query: Query<&GlobalTransform>,
    mut wheel_query: Query<(&Parent, &mut Wheel)>,
    rapier_context: Res<RapierContext>,
) {
    for (chassis, mut wheel) in wheel_query.iter_mut() {
        let Ok(transform) = chassis_query.get(chassis.get()) else {
            continue;
        };

        // transform point into world space.
        let ray_pos = transform.transform_point(wheel.position);
        let ray_dir = transform.down();
        let filter = QueryFilter::new().exclude_collider(chassis.get());

        if let Some((_entity, ray)) = rapier_context.cast_ray_and_get_normal(
            ray_pos,
            ray_dir,
            wheel.max_suspension,
            true,
            filter,
        ) {
            wheel.ratio = 1. - ray.toi / wheel.max_suspension;
            wheel.normal = Some(ray.normal);
        } else {
            wheel.ratio = 0.;
            wheel.normal = None;
        }
    }
}
