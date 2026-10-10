mod components;
mod systems;

use bevy::prelude::*;
use systems::PendulumPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Inverted Pendulum Simulation".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PendulumPlugin)
        .run();
}
