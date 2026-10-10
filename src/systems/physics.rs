use crate::components::{ControlTorque, MotionState, Rod};
use bevy::prelude::*;

pub fn verlet_step(theta: f32, theta_dot: f32, length: f32, dt: f32) -> (f32, f32) {
    const G: f32 = 9.81;

    // Local helper for equation of motion acceleration
    let get_acc = |angle: f32| -> f32 { -(3.0 * G / (2.0 * length)) * angle.sin() };

    // 1. Acceleration at current state
    let theta_ddot = get_acc(theta);

    // 2. Position update
    let theta_new = theta + (theta_dot * dt) + (0.5 * theta_ddot * dt.powi(2));

    // 3. Acceleration at new position
    let theta_ddot_new = get_acc(theta_new);

    // 4. Velocity update using averaged acceleration
    let theta_dot_new = theta_dot + 0.5 * (theta_ddot + theta_ddot_new) * dt;

    (theta_new, theta_dot_new)
}

/// Calculates total angular acceleration given angle, mass, length, and control torque.
pub fn get_total_acceleration(theta: f32, u: f32, mass: f32, length: f32) -> f32 {
    // TODO: extract G into config
    const G: f32 = 9.81;
    let gravity_acc = -(3.0 * G / (2.0 * length)) * theta.sin();
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

pub fn step_motion_state(state: MotionState, length: f32, dt: f32) -> MotionState {
    let (next_theta, next_theta_dot) = verlet_step(state.theta, state.theta_dot, length, dt);

    MotionState {
        theta: next_theta,
        theta_dot: next_theta_dot,
    }
}

pub fn rod_physics_system(time: Res<Time<Fixed>>, mut query: Query<&mut Rod>) {
    let dt = time.delta_secs();
    for mut rod in query.iter_mut() {
        rod.state = step_motion_state(rod.state, rod.length, dt);
    }
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

        rod.state.theta = new_theta;
        rod.state.theta_dot = new_theta_dot;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verlet_step_at_equilibrium() {
        let (theta, theta_dot) = (0.0, 0.0);
        let length = 1.0;
        let dt = 0.01;

        let (new_theta, new_theta_dot) = verlet_step(theta, theta_dot, length, dt);

        assert_eq!(new_theta, 0.0);
        assert_eq!(new_theta_dot, 0.0);
    }

    #[test]
    fn test_verlet_step_restoring_force() {
        // With positive deflection and 0 velocity, gravity should produce negative acceleration
        let theta = 0.5; // ~28.6 degrees
        let theta_dot = 0.0;
        let length = 1.0;
        let dt = 0.01;

        let (new_theta, new_theta_dot) = verlet_step(theta, theta_dot, length, dt);

        // Angle should decrease and angular velocity should become negative (moving towards 0)
        assert!(
            new_theta < theta,
            "Angle should decrease towards equilibrium"
        );
        assert!(
            new_theta_dot < 0.0,
            "Angular velocity should become negative"
        );
    }

    #[test]
    fn test_step_motion_state() {
        let initial_state = MotionState::new(0.3, 0.1);
        let length = 1.5;
        let dt = 0.02;

        let next_state = step_motion_state(initial_state, length, dt);

        let (expected_theta, expected_theta_dot) = verlet_step(0.3, 0.1, length, dt);
        assert_eq!(next_state.theta, expected_theta);
        assert_eq!(next_state.theta_dot, expected_theta_dot);
    }

    // TODO: only test the physics calculation no need to spawn an app
    #[test]
    fn test_rod_physics_system_in_app() {
        let mut app = App::new();
        app.insert_resource(Time::<Fixed>::from_seconds(0.01));
        app.add_systems(Update, rod_physics_system);

        let initial_theta = 0.4;
        let entity = app
            .world_mut()
            .spawn(Rod::new(1.0, 1.0, initial_theta, 0.0))
            .id();

        app.update();

        let rod = app
            .world()
            .get::<Rod>(entity)
            .expect("Rod entity should exist");
        assert!(
            rod.state.theta < initial_theta,
            "Rod theta should have updated and decreased towards 0"
        );
        assert!(
            rod.state.theta_dot < 0.0,
            "Rod theta_dot should have become negative"
        );
    }
}
