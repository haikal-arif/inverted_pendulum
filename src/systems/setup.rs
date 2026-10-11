use std::f32::consts::FRAC_PI_4;

use crate::components::{ControlTorque, PidController, PidGains, PidState, Rod};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;
}

// Some Early PID Data
// Ku = 5
// Tu = 6
// => Kp = 0.6 * 5 = 3
// => Ki = 1.2 * 5 / 6 = 1
// => Kd = 0.075 * 5 * 6 = 2.25

pub fn setup_controlled_rod(mut commands: Commands) {
    commands.spawn((
        Rod::new(1.0, 1.0, FRAC_PI_4, 0.0),
        PidController::new(
            5.0,
            0.0,
            PidGains {
                kp: 7.0,
                ki: 0.3,
                kd: 4.0,
            },
            PidState::default(),
        ),
        ControlTorque::default(),
    ));
}
