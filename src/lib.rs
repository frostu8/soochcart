//! SOOCH CART!

pub mod camera;
pub mod debug;
pub mod kart;
pub mod map;
pub mod random;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

/// The game's plugins.
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(camera::CameraPlugin)
            .add(map::MapPlugin)
            .add(kart::KartPlugin)
            .add(kart::input::InputPlugin)
            .add(kart::wheel::WheelPlugin)
            .add(debug::DebugPlugin)
            .add(random::RandomPlugin)
            .add(GameStatePlugin)
    }
}

/// A plugin for the main game state.
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}

/// The state of the game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// Default state.
    #[default]
    Splash,
    /// Loading a map. See the [`map`] module high-level documentation.
    LoadingMap,
    /// In a track.
    InGame,
}
