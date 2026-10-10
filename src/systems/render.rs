use crate::components::Rod;
use bevy::prelude::*;

pub fn render_rod_system(query: Query<&Rod>, mut gizmos: Gizmos) {
    const PIXELS_PER_METER: f32 = 250.0;
    let pivot = Vec2::new(0.0, 50.0);

    for rod in &query {
        let length_px = rod.length * PIXELS_PER_METER;
        let tip = pivot
            + Vec2::new(
                length_px * rod.state.theta.sin(),
                -length_px * rod.state.theta.cos(),
            );

        // Draw neutral axis reference line (faint vertical guide)
        gizmos.line_2d(
            pivot,
            pivot + Vec2::new(0.0, -length_px * 1.1),
            Color::srgba(1.0, 1.0, 1.0, 0.15),
        );

        // Draw the pendulum rod
        gizmos.line_2d(pivot, tip, Color::srgb(0.95, 0.95, 0.95));

        // Draw the pivot anchor
        gizmos.circle_2d(pivot, 8.0, Color::srgb(0.9, 0.25, 0.25));

        // Draw the pendulum tip bob
        gizmos.circle_2d(tip, 14.0, Color::srgb(0.25, 0.65, 1.0));
    }
}
