pub mod control;
pub mod physics;
pub mod render;
pub mod setup;

pub use control::*;
pub use physics::*;
pub use render::*;
pub use setup::*;

use bevy::prelude::*;

pub struct PendulumPlugin;

impl Plugin for PendulumPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_seconds(0.01))
            .add_systems(Startup, (setup_camera, setup_gizmos, setup_controlled_rod))
            .add_systems(
                FixedUpdate,
                (pid_controller_system, rod_physics_system_with_controller).chain(),
            )
            .add_systems(Update, render_rod_system);
    }
}
