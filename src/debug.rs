//! Debug utilities.
//!
//! The systems in this module are also externally available.

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::kart::LocalPlayer;
use crate::random::Random;

/// Debug utilities.
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_random_impulse);
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
