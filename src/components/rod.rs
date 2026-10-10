use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Rod {
    pub mass: f32,
    pub length: f32,
    pub state: MotionState,
}

impl Rod {
    pub fn new(mass: f32, length: f32, theta: f32, theta_dot: f32) -> Self {
        Self {
            mass,
            length,
            state: MotionState::new(theta, theta_dot),
        }
    }
}

impl Default for Rod {
    fn default() -> Self {
        Self {
            mass: 1.0,
            length: 1.0,
            state: MotionState::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotionState {
    pub theta: f32,
    pub theta_dot: f32,
}

impl MotionState {
    pub fn new(theta: f32, theta_dot: f32) -> Self {
        Self { theta, theta_dot }
    }
}

impl Default for MotionState {
    fn default() -> Self {
        Self {
            theta: 0.0,
            theta_dot: 0.0,
        }
    }
}
