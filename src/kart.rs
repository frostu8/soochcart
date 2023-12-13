//! Kart phyiscs.

use bevy::prelude::*;

use bevy_rapier3d::prelude::*;

use crate::camera::FollowKartBundle;
use crate::GameState;

/// Kart plugin.
pub struct KartPlugin;

impl Plugin for KartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_local_player);
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
    pub external_impulse: ExternalImpulse,
    pub mass_properties: ReadMassProperties,
    pub kart_options: KartOptions,
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
            external_impulse: default(),
            mass_properties: default(),
            kart_options: default(),
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
                transform: Transform::from_xyz(0., 1.5, 0.),
                ..default()
            },
            LocalPlayer,
        ))
        .id();

    commands.spawn((Camera3dBundle::default(), FollowKartBundle::new(kart)));
}
