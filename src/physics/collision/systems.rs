use crate::physics::*;
use crate::prelude::*;
use std::collections::HashSet;
use std::mem;

#[derive(Clone, Copy)]
struct Contact {
    normal: Vec2, // points *from* B to A
    penetration: f32,
}

/// System for circle (dynamic) vs rectangle (static wall) collisions.
pub fn circle_wall_collision_system(
    mut movers: Query<(Entity, &mut PhysicalTranslation, &mut Velocity, &Collider), With<Velocity>>,
    walls: Query<(Entity, &Transform, &Collider), Without<Velocity>>,
    mut contacts: ResMut<Contacts>,
) {
    for (mover, mut pos, mut vel, col) in &mut movers {
        let ColliderShape::Circle { radius } = col.0 else {
            continue;
        };
        let center = Vec2::new(pos.x, pos.y);

        for (wall, tf, wall_col) in &walls {
            let ColliderShape::Rect { half_extents } = wall_col.0 else {
                continue;
            };
            let rect_pos = tf.translation.truncate();

            if let Some(contact) = circle_vs_rect(center, radius, rect_pos, half_extents) {
                resolve_circle_wall(pos.reborrow(), vel.reborrow(), contact);
                contacts.current.insert(ordered_pair(mover, wall));
            }
        }
    }
}

/// System for circle vs circle collisions (between dynamic moving entities).
pub fn circle_circle_collision_system(
    mut q: Query<(
        Entity,
        &mut PhysicalTranslation,
        &mut Velocity,
        &Collider,
        Option<&Mass>,
    )>,
    mut contacts: ResMut<Contacts>,
) {
    let mut combos = q.iter_combinations_mut::<2>();
    while let Some(
        [
            (e1, mut p1, mut v1, col1, m1_opt),
            (e2, mut p2, mut v2, col2, m2_opt),
        ],
    ) = combos.fetch_next()
    {
        let ColliderShape::Circle { radius: r1 } = col1.0 else {
            continue;
        };
        let ColliderShape::Circle { radius: r2 } = col2.0 else {
            continue;
        };

        if let Some(contact) =
            circle_vs_circle(Vec2::new(p1.x, p1.y), r1, Vec2::new(p2.x, p2.y), r2)
        {
            let m1 = m1_opt.map_or(1.0, |m| m.0);
            let m2 = m2_opt.map_or(1.0, |m| m.0);

            resolve_circle_circle(
                p1.reborrow(),
                v1.reborrow(),
                m1,
                p2.reborrow(),
                v2.reborrow(),
                m2,
                contact,
            );

            contacts.current.insert(ordered_pair(e1, e2));
        }
    }
}

pub fn emit_collision_events(
    mut contacts: ResMut<Contacts>,
    mut writer: EventWriter<CollisionEvent>,
) {
    let current: HashSet<(Entity, Entity)> = mem::take(&mut contacts.current);

    for &pair in current.difference(&contacts.prev) {
        writer.write(CollisionEvent::Started(pair.0, pair.1));
    }
    for &pair in contacts.prev.difference(&current) {
        writer.write(CollisionEvent::Stopped(pair.0, pair.1));
    }

    contacts.prev = current;
}

//--------------------------------------------------
// Helper functions
//--------------------------------------------------

fn ordered_pair(a: Entity, b: Entity) -> (Entity, Entity) {
    if a.index() < b.index() {
        (a, b)
    } else {
        (b, a)
    }
}

fn circle_vs_rect(circle_pos: Vec2, radius: f32, rect_pos: Vec2, half: Vec2) -> Option<Contact> {
    // closest point on the rectangle to the circle centre
    let delta = circle_pos - rect_pos;
    let clamped = delta.clamp(-half, half);
    let closest = rect_pos + clamped;

    let diff = circle_pos - closest;
    let dist_sq = diff.length_squared();
    if dist_sq > radius * radius {
        return None; // no overlap
    }

    let dist = dist_sq.sqrt();
    let normal = if dist != 0.0 {
        diff / dist
    } else if delta.x.abs() > delta.y.abs() {
        Vec2::new(delta.x.signum(), 0.0)
    } else {
        Vec2::new(0.0, delta.y.signum())
    };
    Some(Contact {
        normal,
        penetration: radius - dist,
    })
}

fn circle_vs_circle(p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> Option<Contact> {
    let diff = p2 - p1;
    let dist_sq = diff.length_squared();
    let r_sum = r1 + r2;
    if dist_sq >= r_sum * r_sum {
        return None;
    }
    let dist = dist_sq.sqrt();
    let normal = if dist > 0.0 {
        diff / dist
    } else {
        Vec2::ONE.normalize()
    };
    Some(Contact {
        normal,
        penetration: r_sum - dist,
    })
}

fn resolve_circle_wall(
    mut circle_pos: Mut<PhysicalTranslation>,
    mut circle_vel: Mut<Velocity>,
    contact: Contact,
) {
    // elastic bounce
    let vel2d = Vec2::new(circle_vel.x, circle_vel.y);
    let v_dot_n = vel2d.dot(contact.normal);
    let reflect = vel2d - 2.0 * v_dot_n * contact.normal;
    circle_vel.x = reflect.x;
    circle_vel.y = reflect.y;

    circle_pos.x += contact.normal.x * contact.penetration;
    circle_pos.y += contact.normal.y * contact.penetration;
}

fn resolve_circle_circle(
    mut pos1: Mut<PhysicalTranslation>,
    mut vel1: Mut<Velocity>,
    m1: f32,
    mut pos2: Mut<PhysicalTranslation>,
    mut vel2: Mut<Velocity>,
    m2: f32,
    contact: Contact,
) {
    // elastic exchange (vector form)
    let v1 = Vec2::new(vel1.x, vel1.y);
    let v2 = Vec2::new(vel2.x, vel2.y);
    let n = contact.normal;

    let v1n = v1 - (2.0 * m2 / (m1 + m2)) * (v1 - v2).dot(n) * n;
    let v2n = v2 - (2.0 * m1 / (m1 + m2)) * (v2 - v1).dot(n) * n;

    vel1.x = v1n.x;
    vel1.y = v1n.y;
    vel2.x = v2n.x;
    vel2.y = v2n.y;

    let total = m1 + m2;
    let corr1 = contact.penetration * (m2 / total);
    let corr2 = contact.penetration * (m1 / total);

    pos1.x -= n.x * corr1;
    pos1.y -= n.y * corr1;
    pos2.x += n.x * corr2;
    pos2.y += n.y * corr2;
}
