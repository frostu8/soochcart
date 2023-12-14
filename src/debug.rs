//! Debug utilities.
//!
//! The systems in this module are also externally available.

use bevy::prelude::*;
use bevy::transform::TransformSystem;

use bevy_rapier3d::prelude::*;

use crate::kart::{wheel::Wheel, LocalPlayer};
use crate::random::Random;

/// Debug utilities.
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (debug_random_impulse, debug_draw_wheels).after(TransformSystem::TransformPropagate),
        );
    }
}

/// Applies a random impulse on the local kart that lifts it up.
pub fn debug_random_impulse(
    mut local_kart_query: Query<(&mut ExternalImpulse, &ReadMassProperties), With<LocalPlayer>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut rng: ResMut<Random>,
) {
    let Ok((mut ei, mass_properties)) = local_kart_query.get_single_mut() else {
        return;
    };

    let mass_properties = mass_properties.get();

    if keyboard_input.just_pressed(KeyCode::R) {
        *ei = ExternalImpulse::at_point(
            Vec3::new(0., 3. * mass_properties.mass, 0.),
            rng.in_sphere(1.),
            mass_properties.local_center_of_mass,
        );
    }
}

pub fn debug_draw_wheels(
    chassis_query: Query<&GlobalTransform>,
    wheels_query: Query<(&Parent, &Wheel, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (parent, wheel, wheel_transform) in wheels_query.iter() {
        let Ok(transform) = chassis_query.get(parent.get()) else {
            continue;
        };

        let ray_pos = transform.transform_point(wheel.position);
        let ray_dir = transform.down() * wheel.max_suspension * (1. - wheel.ratio());

        let color = if wheel.ratio() > 0. {
            Color::GREEN
        } else {
            Color::RED
        };

        gizmos.line(ray_pos, ray_pos + ray_dir, color);

        gizmos.sphere(wheel_transform.translation(), Quat::IDENTITY, 0.02, color);
    }
}
