use std::f32::consts::{FRAC_PI_4, PI};

use crate::components::{ControlTorque, PidController, PidGains, PidState, Rod};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 4.0;
}

pub fn setup_controlled_rod(mut commands: Commands) {
    commands.spawn((
        Rod::new(1.0, 1.0, PI + 0.5, 0.0),
        PidController::new(
            1.0,
            PI,
            PidGains {
                kp: 5.0,
                ki: 1.0,
                kd: 0.0,
            },
            PidState::default(),
        ),
        ControlTorque::default(),
    ));
}
