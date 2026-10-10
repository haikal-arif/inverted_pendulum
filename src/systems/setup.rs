use crate::components::{ControlTorque, PidController, Rod};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;
}

pub fn setup_rod(mut commands: Commands) {
    // Spawn a rod with an initial deflection angle (e.g. 45 degrees / pi/4) so it swings
    commands.spawn(Rod::new(1.0, 1.0, std::f32::consts::FRAC_PI_4 * 0.1, 0.0));
}

// TODO: use sensible starting points, too lazy to input now
pub fn setup_controlled_rod(mut commands: Commands) {
    commands.spawn((
        Rod::default(),
        PidController::default(),
        ControlTorque::default(),
    ));
}
