//! Kart phyiscs.

pub mod wheel;

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::camera::FollowKartBundle;
use crate::GameState;

use wheel::{Wheel, WheelBundle};

/// Kart plugin.
pub struct KartPlugin;

impl Plugin for KartPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Suspension>()
            .add_systems(OnEnter(GameState::InGame), spawn_local_player)
            .add_systems(FixedUpdate, propagate_suspension)
            .add_systems(Update, create_kart_wheels);
    }
}

/// Kart bundle.
#[derive(Bundle, Clone, Debug)]
pub struct KartBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub collider_mass_properties: ColliderMassProperties,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub velocity: Velocity,
    pub mass_properties: ReadMassProperties,
    pub kart_options: KartOptions,
    pub suspension: Suspension,
    pub kart: Kart,
}

impl Default for KartBundle {
    fn default() -> KartBundle {
        KartBundle {
            transform: default(),
            global_transform: default(),
            visibility: default(),
            inherited_visibility: default(),
            view_visibility: default(),
            rigidbody: RigidBody::Dynamic,
            // TODO tweak as needed
            collider: Collider::cuboid(0.35, 0.166, 0.5),
            collider_mass_properties: ColliderMassProperties::Mass(20.),
            external_force: default(),
            external_impulse: default(),
            velocity: default(),
            mass_properties: default(),
            kart_options: default(),
            suspension: default(),
            kart: default(),
        }
    }
}

/// Kart movement options.
#[derive(Clone, Component, Debug)]
pub struct KartOptions {
    /// How fast the kart accelerates in m/s^3.
    ///
    /// Mass is ignored.
    pub acceleration: f32,
}

impl Default for KartOptions {
    fn default() -> KartOptions {
        KartOptions { acceleration: 5. }
    }
}

/// Kart suspension.
///
/// Copies its properties to the children [`Wheel`]s.
#[derive(Clone, Component, Debug, Reflect)]
pub struct Suspension {
    /// The maximum distance for suspension.
    pub max_suspension: f32,
    /// The maximum force applied by the suspension.
    pub max_force: f32,
    pub damping_factor: f32,
}

impl Default for Suspension {
    fn default() -> Suspension {
        Suspension {
            max_suspension: 0.4,
            max_force: 225.,
            damping_factor: 0.2,
        }
    }
}

/// Manipulates the physics of the kart to act karty.
#[derive(Clone, Component, Debug, Default)]
pub struct Kart;

/// A marker component for the local player.
#[derive(Clone, Component, Debug, Default)]
pub struct LocalPlayer;

fn spawn_local_player(mut commands: Commands) {
    let kart = commands
        .spawn((
            KartBundle {
                transform: Transform::from_xyz(-8., 7.5, 0.),
                ..default()
            },
            LocalPlayer,
        ))
        .id();

    commands.spawn((Camera3dBundle::default(), FollowKartBundle::new(kart)));
}

fn create_kart_wheels(mut commands: Commands, new_karts_query: Query<Entity, Added<Kart>>) {
    for kart in new_karts_query.iter() {
        //Collider::cuboid(0.35, 0.166, 0.5)
        commands.entity(kart).with_children(|parent| {
            // front right wheel
            parent.spawn(WheelBundle::new(Vec3::new(0.35, -0.16, 0.5)));
            // front left wheel
            parent.spawn(WheelBundle::new(Vec3::new(-0.35, -0.16, 0.5)));
            // back right wheel
            parent.spawn(WheelBundle::new(Vec3::new(0.35, -0.16, -0.5)));
            // back left wheel
            parent.spawn(WheelBundle::new(Vec3::new(-0.35, -0.16, -0.5)));
        });
    }
}

fn propagate_suspension(
    suspension_query: Query<(&Suspension, &Children)>,
    mut wheels_query: Query<&mut Wheel>,
) {
    for (suspension, children) in suspension_query.iter() {
        let mut wheels = wheels_query.iter_many_mut(children);

        while let Some(mut wheel) = wheels.fetch_next() {
            wheel.max_force = suspension.max_force;
            wheel.max_suspension = suspension.max_suspension;
            wheel.damping_factor = suspension.damping_factor;
        }
    }
}
