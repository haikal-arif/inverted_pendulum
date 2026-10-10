use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct PidController {
    pub target_theta: f32,
    pub gains: PidGains,
    pub state: PidState,
}

#[derive(Component, Default, Clone, Copy)]
pub struct PidGains {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

#[derive(Component, Default, Clone, Copy)]
pub struct PidState {
    pub integral: f32,
    pub prev_error: f32,
}

#[derive(Component, Default)]
pub struct ControlTorque(pub f32);
