use std::{f32::consts::FRAC_PI_2, time::UNIX_EPOCH};

use crate::components::{ControlTorque, Rod};
use bevy::prelude::*;

/// Calculates total angular acceleration given angle, mass, length, and control torque.
pub fn get_total_acceleration(theta: f32, u: f32, mass: f32, length: f32) -> f32 {
    // TODO: extract G into config
    const G: f32 = 9.81;
    let gravity_acc = (3.0 * G / (2.0 * length)) * theta.sin();
    let control_acc = (3.0 * u) / (mass * length.powi(2));

    gravity_acc + control_acc
}

pub fn verlet_step_with_torque(
    theta: f32,
    theta_dot: f32,
    u: f32,
    mass: f32,
    length: f32,
    dt: f32,
) -> (f32, f32) {
    // 1. Current acceleration with torque
    let theta_ddot = get_total_acceleration(theta, u, mass, length);

    // 2. Position update
    let theta_new = theta + (theta_dot * dt) + (0.5 * theta_ddot * dt.powi(2));

    // 3. Acceleration at new position with same torque
    let theta_ddot_new = get_total_acceleration(theta_new, u, mass, length);

    // 4. Velocity update
    let theta_dot_new = theta_dot + 0.5 * (theta_ddot + theta_ddot_new) * dt;

    (theta_new, theta_dot_new)
}

pub fn rod_physics_system_with_controller(
    time: Res<Time<Fixed>>,
    mut query: Query<(&mut Rod, &ControlTorque)>,
) {
    let dt = time.delta_secs();

    for (mut rod, torque) in query.iter_mut() {
        // Pure Velocity Verlet step
        let (new_theta, new_theta_dot) = verlet_step_with_torque(
            rod.state.theta,
            rod.state.theta_dot,
            torque.0,
            rod.mass,
            rod.length,
            dt,
        );

        if new_theta_dot > 0.0 && rod.state.theta_dot <= 0.0 {
            let start = std::time::SystemTime::now();
            let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            println!("One wave reached time: {:?}", since_epoch);
            println!("Amplitude: {:?}", new_theta);
        }
        rod.state.theta = (-FRAC_PI_2).max(new_theta.min(FRAC_PI_2));
        rod.state.theta_dot = new_theta_dot;
    }
}
