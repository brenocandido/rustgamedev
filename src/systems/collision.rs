use crate::components::*;
use bevy::prelude::*;

pub fn circle_rect_collider(
    mut movers: Query<(&mut Transform, &mut Velocity, &CircleCollider), With<Player>>,
    statics: Query<(&Transform, &RectCollider), Without<Player>>,
) {
    for (mut tf, mut vel, circle) in &mut movers {
        //----------------------------------------------------------------------
        // 2) For each wall: test & resolve
        //----------------------------------------------------------------------
        // World-space circle centre & radius
        let centre = tf.translation.truncate();
        let radius = circle.radius * tf.scale.x; // assume uniform scale

        for (w_tf, rect) in &statics {
            // World-space box centre & half-extents (apply scale if any)
            let box_center = w_tf.translation.truncate();
            let box_he = rect.half_extents * w_tf.scale.truncate();

            // Closest point on the AABB to the circle centre
            let closest = (centre - box_center).clamp(-box_he, box_he) + box_center;

            // Vector from closest point to centre
            let delta = centre - closest;
            let dist2 = delta.length_squared();

            if dist2 < radius * radius {
                // ---------- overlap! ----------
                let dist = dist2.sqrt();
                // If circle centre is *inside* the box (dist == 0) push it out vertically
                let penetration_depth = radius - dist;
                let push_dir = if dist > 0.0001 { delta / dist } else { Vec2::Y };

                // Move circle out & zero velocity along the push axis
                tf.translation += (push_dir * penetration_depth).extend(0.0);

                if push_dir.x.abs() > push_dir.y.abs() {
                    if push_dir.x >= 0.0 {
                        vel.0.x = f32::max(0.0, vel.0.x);
                    } else {
                        vel.0.x = f32::min(0.0, vel.0.x);
                    }
                } else if push_dir.y >= 0.0 {
                    vel.0.y = f32::max(0.0, vel.0.y);
                } else {
                    vel.0.y = f32::min(0.0, vel.0.y);
                }

                // input.0 = Vec2::ZERO;
            }
        }
    }
}
