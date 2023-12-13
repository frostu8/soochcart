//! Random utilities.

use bevy::math::{Quat, Vec3};
use bevy::prelude::{App, Plugin, Resource};

use rand::{rngs::StdRng, Rng, SeedableRng};

use std::f32::consts::PI;

/// The random plugin.
///
/// Simply instantiates the [`Random`] resource.
pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Random>();
    }
}

/// The random resource.
///
/// Use this to get awesome rngs.
#[derive(Resource)]
pub struct Random {
    rng: StdRng,
}

impl Random {
    /// Gets a random number between `[0, 1)`.
    pub fn real(&mut self) -> f32 {
        self.rng.gen()
    }

    /// Gets a random point in a sphere.
    pub fn in_sphere(&mut self, radius: f32) -> Vec3 {
        let rot = Quat::from_axis_angle(Vec3::X, self.real() * 2. * PI)
            * Quat::from_axis_angle(Vec3::Y, self.real() * 2. * PI)
            * Quat::from_axis_angle(Vec3::Z, self.real() * 2. * PI);

        let forward = rot * Vec3::Z;

        forward * self.real() * radius
    }
}

impl Default for Random {
    fn default() -> Random {
        Random {
            rng: StdRng::from_entropy(),
        }
    }
}
