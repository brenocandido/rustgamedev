use crate::components::*;
use bevy::prelude::*;

/// System for circle (dynamic) vs rectangle (static wall) collisions.
pub fn circle_wall_collision_system(
    mut query_movers: Query<(Entity, &mut PhysicalTranslation, &mut Velocity, &Collider), With<Velocity>>, 
    // ^ selects entities with Velocity (i.e., dynamic circles) 
    query_walls: Query<(Entity, &Transform, &Collider), Without<Velocity>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mover_entity, mut phys_pos, mut vel, mover_collider) in &mut query_movers {
        // Only proceed if mover has a circle collider
        if let ColliderShape::Circle { radius } = mover_collider.0 {
            let circle_center = Vec2::new(phys_pos.x, phys_pos.y);
            // Check collision against each wall
            for (wall_entity, wall_tf, wall_collider) in &query_walls {
                if let ColliderShape::Rect { half_extents } = wall_collider.0 {
                    let rect_center = Vec2::new(wall_tf.translation.x, wall_tf.translation.y);
                    let half = half_extents;
                    // Compute the closest point on the rectangle to the circle's center
                    let delta = circle_center - rect_center;
                    let clamped = Vec2::new(
                        delta.x.clamp(-half.x, half.x),
                        delta.y.clamp(-half.y, half.y),
                    );
                    let closest_point = rect_center + clamped;
                    // Compute distance from circle center to this point
                    let diff = circle_center - closest_point;
                    let dist_sq = diff.length_squared();
                    if dist_sq <= radius * radius {
                        // Collision detected
                        let dist = dist_sq.sqrt();
                        // Compute collision normal (from wall to circle)
                        let normal = if dist != 0.0 {
                            diff / dist
                        } else {
                            // Circle center is exactly on the edge/corner: use outward normal
                            // (choose based on which side is closest)
                            if delta.x.abs() > delta.y.abs() {
                                Vec2::new(delta.x.signum(), 0.0)
                            } else {
                                Vec2::new(0.0, delta.y.signum())
                            }
                        };
                        // Reflect velocity along the collision normal (elastic bounce)
                        let vel2d = Vec2::new(vel.x, vel.y);
                        let v_dot_n = vel2d.dot(normal);
                        let reflected = vel2d - 2.0 * v_dot_n * normal;
                        vel.x = reflected.x;
                        vel.y = reflected.y;
                        // Slightly reposition the circle outside the wall to avoid sticking
                        let penetration = radius - dist;
                        if penetration > 0.0 {
                            // Move circle out by penetration depth along the normal
                            phys_pos.x += normal.x * penetration;
                            phys_pos.y += normal.y * penetration;
                        }
                        // Emit collision event
                        collision_events.write(CollisionEvent(mover_entity, wall_entity));
                    }
                }
            }
        }
    }
}

/// System for circle vs circle collisions (between dynamic moving entities).
pub fn circle_circle_collision_system(
    mut query: Query<(Entity, &mut PhysicalTranslation, &mut Velocity, &Collider, Option<&Mass>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut combos = query.iter_combinations_mut::<2>(); 
    while let Some([(ent1, mut pos1, mut vel1, col1, mass1_opt), (ent2, mut pos2, mut vel2, col2, mass2_opt)]) = combos.fetch_next() {
        // Only handle if both have Circle colliders
        let radius1 = if let ColliderShape::Circle { radius } = col1.0 { radius } else { continue };
        let radius2 = if let ColliderShape::Circle { radius } = col2.0 { radius } else { continue };

        // 2D positions of centers
        let p1 = Vec2::new(pos1.x, pos1.y);
        let p2 = Vec2::new(pos2.x, pos2.y);
        let diff = p2 - p1;
        let dist_sq = diff.length_squared();
        let radius_sum = radius1 + radius2;
        if dist_sq < radius_sum * radius_sum {
            // Circles overlap -> collision
            let dist = dist_sq.sqrt();
            let normal = if dist != 0.0 {
                diff / dist  // unit vector from ent1 towards ent2
            } else {
                Vec2::ONE.normalize()  // in case they exactly overlap, choose an arbitrary normal
            };

            // Determine masses (default to 1.0 if not specified)
            let m1 = mass1_opt.map_or(1.0, |m| m.0);
            let m2 = mass2_opt.map_or(1.0, |m| m.0);

            // Velocities in 2D for calculation
            let v1 = Vec2::new(vel1.x, vel1.y);
            let v2 = Vec2::new(vel2.x, vel2.y);

            // Compute new velocities after elastic collision (vector form)
            let v1_new = v1 - (2.0 * m2 / (m1 + m2)) * ((v1 - v2).dot(normal)) * normal;
            let v2_new = v2 - (2.0 * m1 / (m1 + m2)) * ((v2 - v1).dot(normal)) * normal;

            // Update the velocity components
            vel1.x = v1_new.x;
            vel1.y = v1_new.y;
            vel2.x = v2_new.x;
            vel2.y = v2_new.y;

            // Separate the circles to remove overlap (move each out by half the penetration)
            let penetration = if dist != 0.0 { radius_sum - dist } else { radius_sum };
            let correction = 0.5 * penetration;
            // Move ent1 opposite to normal, ent2 along normal
            pos1.x -= normal.x * correction * (m2 / (m1 + m2));
            pos1.y -= normal.y * correction * (m2 / (m1 + m2));
            pos2.x += normal.x * correction * (m1 / (m1 + m2));
            pos2.y += normal.y * correction * (m1 / (m1 + m2));

            // Emit collision event
            collision_events.write(CollisionEvent(ent1, ent2));
        }
    }
}