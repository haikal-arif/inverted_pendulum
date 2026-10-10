use crate::{
    components::{ControlTorque, PidController, PidGains, PidState, Rod},
    systems::verlet_step_with_torque,
};
use bevy::prelude::*;

pub fn pid_controller_system(
    time: Res<Time<Fixed>>,
    mut query: Query<(&Rod, &mut PidController, &mut ControlTorque)>,
) {
    let dt = time.delta_secs();

    for (rod, mut controller, mut torque) in query.iter_mut() {
        // Pure PID calculation
        let (computed_torque, new_state) = pid_step(
            rod.state.theta,
            controller.target_theta,
            controller.state,
            controller.gains,
            dt,
        );

        // Update controller state
        controller.state = new_state;

        // Write output to the bridge component
        torque.0 = computed_torque;
    }
}

pub fn pid_step(
    current_angle: f32,
    target_angle: f32,
    state: PidState,
    gains: PidGains,
    dt: f32,
) -> (f32, PidState) {
    let error = target_angle - current_angle;

    // Accumulate integral with anti-windup clamping if needed
    let new_integral = state.integral + error * dt;

    // Derivative of error
    let derivative = if dt > 0.0 {
        (error - state.prev_error) / dt
    } else {
        0.0
    };

    // PID torque output: u = Kp*e + Ki*∫e dt + Kd*(de/dt)
    let u = (gains.kp * error) + (gains.ki * new_integral) + (gains.kd * derivative);

    let next_state = PidState {
        integral: new_integral,
        prev_error: error,
    };

    (u, next_state)
}
