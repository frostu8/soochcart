use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use soochcart::map::LoadMap;
use soochcart::{GamePlugins, GameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(GamePlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut load_map: ResMut<LoadMap>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    load_map.map = asset_server.load("maps/testing/scene.glb#Scene0");

    next_state.set(GameState::LoadingMap);
}
