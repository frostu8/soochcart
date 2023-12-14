//! Kart phyiscs.

pub mod input;
pub mod wheel;

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::camera::FollowKartBundle;
use crate::GameState;

use wheel::{Wheel, WheelBundle, WheelSystem};
use input::{PlayerCommands, InputSystem};

/// Kart plugin.
pub struct KartPlugin;

impl Plugin for KartPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Chassis>()
            .register_type::<KartOptions>()
            .add_systems(OnEnter(GameState::InGame), spawn_local_player)
            .add_systems(
                FixedUpdate,
                propagate_chassis_properties.before(WheelSystem::Raycast),
            )
            .add_systems(
                FixedUpdate,
                average_chassis_normals.after(WheelSystem::Raycast),
            )
            .add_systems(
                FixedUpdate,
                apply_chassis_acceleration
                    .after(WheelSystem::Raycast)
                    .after(InputSystem::Collect)
                    .after(KartSystem::ResetForces),
            )
            .add_systems(
                FixedUpdate,
                reset_chassis_forces.in_set(KartSystem::ResetForces),
            )
            .add_systems(Update, create_kart_wheels);
    }
}

/// Kart systems.
#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum KartSystem {
    /// Resets the external forces acting on a kart.
    ResetForces,
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
    pub chassis: Chassis,
    pub player_commands: PlayerCommands,
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
            chassis: default(),
            player_commands: default(),
        }
    }
}

/// Kart movement options.
#[derive(Clone, Component, Debug, Reflect)]
pub struct KartOptions {
    /// How fast the kart accelerates in m/s^3.
    ///
    /// Mass is ignored.
    pub max_acceleration: f32,
    /// The maximum velocity the kart can achieve alone.
    pub max_velocity: f32,
}

impl Default for KartOptions {
    fn default() -> KartOptions {
        KartOptions {
            max_acceleration: 30.,
            max_velocity: 12.,
        }
    }
}

/// Kart chassis physics properties.
///
/// Copies its properties to the children [`Wheel`]s.
#[derive(Clone, Component, Debug, Reflect)]
pub struct Chassis {
    /// The maximum distance for suspension.
    pub max_suspension: f32,
    /// The maximum force applied by the suspension.
    pub max_force: f32,
    /// The damping factor of the suspension.
    pub damping_factor: f32,
    wheels_contacting_ground: usize,
    ground_normal: Option<Vec3>,
}

impl Chassis {
    /// The average normal of the ground.
    pub fn ground_normal(&self) -> Option<Vec3> {
        self.ground_normal
    }
}

impl Default for Chassis {
    fn default() -> Chassis {
        Chassis {
            max_suspension: 0.35,
            max_force: 4.,
            damping_factor: 0.2,
            wheels_contacting_ground: 0,
            ground_normal: None,
        }
    }
}

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

fn create_kart_wheels(mut commands: Commands, new_karts_query: Query<Entity, Added<Chassis>>) {
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

fn reset_chassis_forces(mut chassis_query: Query<&mut ExternalForce, With<Chassis>>) {
    for mut ef in chassis_query.iter_mut() {
        *ef = ExternalForce::default();
    }
}

fn propagate_chassis_properties(
    chassis_query: Query<(&Chassis, &Children)>,
    mut wheels_query: Query<&mut Wheel>,
) {
    for (chassis, children) in chassis_query.iter() {
        let mut wheels = wheels_query.iter_many_mut(children);

        while let Some(mut wheel) = wheels.fetch_next() {
            wheel.max_force = chassis.max_force;
            wheel.max_suspension = chassis.max_suspension;
            wheel.damping_factor = chassis.damping_factor;
        }
    }
}

fn average_chassis_normals(
    mut chassis_query: Query<(&mut Chassis, &Children)>,
    wheels_query: Query<&Wheel>,
) {
    for (mut chassis, children) in chassis_query.iter_mut() {
        chassis.wheels_contacting_ground = wheels_query
            .iter_many(children)
            .filter(|w| w.normal().is_some())
            .count();

        let total_normal = wheels_query
            .iter_many(children)
            .filter_map(|w| w.normal())
            .reduce(|acc, x| acc + x);

        chassis.ground_normal = total_normal
            .map(|n| n / chassis.wheels_contacting_ground as f32);
    }
}

fn apply_chassis_acceleration(
    mut chassis_query: Query<(
        &mut ExternalForce,
        &GlobalTransform,
        &Velocity,
        &ReadMassProperties,
        &Chassis,
        &PlayerCommands,
        &KartOptions,
    )>,
    //time: Res<Time>,
) {
    for (mut ef, transform, velocity, mass_properties, chassis, player_commands, options) in chassis_query.iter_mut() {
        // get normal, only apply acceleration if it is grounded
        let Some(ground_normal) = chassis.ground_normal() else {
            continue;
        };

        let mass_properties = mass_properties.get();
        let acceleration = player_commands.commands().acceleration;

        // get projections
        let z_axis = project_on_ground_plane(ground_normal, transform.forward()).normalize();

        // get current velocity in forward direction
        let forward_velocity = z_axis.dot(velocity.linvel);

        let force = (options.max_acceleration * mass_properties.mass) * acceleration;

        if (force < 0. && forward_velocity > -options.max_velocity)
            || (force > 0. && forward_velocity < options.max_velocity)
        {
            *ef += ExternalForce {
                force: z_axis * force,
                ..default()
            };
        }
    }
}

/// Projects a vector onto the ground plane.
pub fn project_on_ground_plane(normal: Vec3, vector: Vec3) -> Vec3 {
    vector - normal * vector.dot(normal)
}

