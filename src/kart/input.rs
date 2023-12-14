//! Kart input systems.

use bevy::prelude::*;

/// Kart input plugin.
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, collect_local_inputs.in_set(InputSystem::Collect));
    }
}

/// System for inputs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, SystemSet)]
pub enum InputSystem {
    /// Collects inputs.
    Collect,
}

/// A single kart's commands.
#[derive(Clone, Component, Debug, Default)]
pub struct PlayerCommands {
    commands: Commands,
}

impl PlayerCommands {
    /// The current inputs for this frame.
    ///
    /// Must be read after [`InputSystem::Collect`].
    pub fn commands(&self) -> Commands {
        self.commands.clone()
    }
}

/// The list of inputs a player can have on a frame.
///
/// The [`Default`] implementation of this is commands that do nothing.
#[derive(Clone, Debug, Default)]
pub struct Commands {
    /// The acceleration of the input.
    ///
    /// `1.` is maximum forward acceleration, `-1.` is maximum backward
    /// acceleration.
    pub acceleration: f32,
}

fn collect_local_inputs(
    mut collector_query: Query<&mut PlayerCommands>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut collector in collector_query.iter_mut() {
        // reset inputs
        collector.commands = Commands::default();

        // TODO gamepad support
        if keyboard_input.pressed(KeyCode::W) {
            collector.commands.acceleration += 1.;
        }

        if keyboard_input.pressed(KeyCode::S) {
            collector.commands.acceleration -= 1.;
        }
    }
}

